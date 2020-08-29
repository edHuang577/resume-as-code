#!/usr/bin/env python
"""
    An ETL for parsing AWS site access logs into partitioned parquet files.

    Requirements (Python 3.7+):
    - boto3
    - pandas
    - pyarrow
    - s3fs
"""
from __future__ import annotations

import argparse
import gzip
import logging
import multiprocessing
import re
from datetime import datetime
from io import BytesIO
from typing import List, Tuple

import boto3
import pandas as pd
from botocore.client import BaseClient


# Used to parse datetime from log names
LOG_NAME_PATTERN = re.compile(r'\.(\d{4}-\d{2}-\d{2}-\d{2})\.')


def parse_args() -> argparse.Namespace:
    """ Parse command line arguments """
    parser = argparse.ArgumentParser()
    parser.add_argument(
        '--bucket', '-b',
        type=str,
        required=True,
        help='Name of S3 bucket where logs are stored',
        metavar='S3_BUCKET'
    )
    parser.add_argument(
        '--source', '-s',
        type=str,
        required=True,
        help='Prefix of raw data directory in the S3 bucket',
        metavar='SOURCE_PREFIX'
    )
    parser.add_argument(
        '--target', '-t',
        type=str,
        required=True,
        help='Prefix directory to save output parquet data in the S3 bucket',
        metavar='TARGET_PREFIX'
    )
    parser.add_argument(
        '--num-workers', '-n',
        type=int,
        default=4,
        help='Number of parallel workers to use for log parsing',
        metavar='NUM_WORKERS'
    )
    parser.add_argument(
        '--start-date', '-S',
        type=lambda x: datetime.strptime(x, '%Y-%m-%d'),
        help='Only load logs on and after this date',
        metavar='YYYY-MM-DD'
    )
    parser.add_argument(
        '--end-date', '-E',
        type=lambda x: datetime.strptime(x, '%Y-%m-%d'),
        help='Only load logs before this date',
        metavar='YYYY-MM-DD'
    )
    return parser.parse_args()


class S3Bucket:
    """ Object for handling S3 bucket interactions """

    def __init__(self, client: BaseClient, bucket: str):
        self.client = client
        self.bucket = bucket

    @classmethod
    def from_env(cls, bucket: str) -> S3Bucket:
        """ Create bucket with default client from environment """
        return cls(boto3.client('s3'), bucket)

    def list_objects(self, prefix: str) -> List[str]:
        """ List objects in bucket matching prefix """
        pages = (
            self.client
            .get_paginator('list_objects_v2')
            .paginate(Bucket=self.bucket, Prefix=prefix)
        )
        return [
            obj.get('Key')
            for page in pages
            for obj in page.get('Contents') or list()
        ]

    def download_bytes(self, key: str) -> bytes:
        """ Download a key from S3 as raw bytes """
        buf = BytesIO()
        self.client.download_fileobj(self.bucket, key, buf)
        return buf.getvalue()

    def write_parquet(self, data: pd.DataFrame, key: str):
        """ Write a DataFrame to S3 as parquet """
        key = key.lstrip('/')
        data.to_parquet(f's3://{self.bucket}/{key}', compression='snappy')


def unpack_gzip(raw: bytes) -> str:
    """ Decompress gzipped data and UTF8 decode """
    return gzip.decompress(raw).decode()


def log_to_frame(content: str) -> pd.DataFrame:
    """ Extract text log data into a DataFrame """
    lines = content.splitlines()[1:]
    return pd.DataFrame(
        map(lambda x: x.split('\t'), lines[1:]),
        columns=lines[0][9:].split(' ')
    )


def log_name_to_datetime(name: str) -> datetime:
    """ Get the datetime of a log from it's name """
    match = LOG_NAME_PATTERN.search(name)
    if not match:
        raise ValueError(f"Invalid log name '{name}'")
    return datetime.strptime(match[1], '%Y-%m-%d-%H')


def load_log(bucket_name: str, key: str, target_prefix: str):
    """ Parse a single log file on S3

        Load:
            s3://[bucket]/key -> s3://[bucket]/[target_prefix]/...

    """
    logging.info(f'Loading s3://{bucket_name}/{key}')
    target_prefix = target_prefix.strip('/')
    bucket = S3Bucket.from_env(bucket_name)

    raw = bucket.download_bytes(key)
    content = unpack_gzip(raw)
    frame = log_to_frame(content)

    log_date = log_name_to_datetime(key)

    year = log_date.strftime('YEAR=%Y')
    month = log_date.strftime('MONTH=%m')
    day = log_date.strftime('DAY=%d')
    basename = log_date.strftime('%H:%M:%S.parquet.snappy')
    target_key = f'{target_prefix}/{year}/{month}/{day}/{basename}'
    bucket.write_parquet(frame, target_key)
    logging.info(f'Saved s3://{bucket_name}/{target_key}')


def load_log_wrapper(args: Tuple[str, str, str]):
    """ Multiprocessing wrapper for 'load_log' function to unpack args """
    return load_log(*args)


def filter_logs(objects: List[str],
                start_date: datetime = None,
                end_date: datetime = None) -> List[str]:
    """ Filter S3 objects for logs within date range """
    filtered = list()
    for obj in objects:
        if not obj.endswith('.gz'):
            continue
        if start_date or end_date:
            log_date = log_name_to_datetime(obj)
            if start_date and log_date < start_date:
                continue
            if end_date and log_date >= end_date:
                continue
        filtered.append(obj)
    return filtered


def main(args: argparse.Namespace):
    """ Main execution """
    fmt = '[%(levelname)s %(asctime)s] %(message)s'
    logging.basicConfig(format=fmt, level=logging.INFO)

    bucket = S3Bucket.from_env(args.bucket)
    objects = bucket.list_objects(args.source)
    logs = filter_logs(objects, args.start_date, args.end_date)

    logging.info(f'Loading {len(logs)} logs')
    tasks = [
        (args.bucket, log, args.target)
        for log in logs
    ]
    pool = multiprocessing.Pool(args.num_workers)
    _ = list(pool.imap_unordered(load_log_wrapper, tasks))
    logging.info("Done.")


if __name__ == '__main__':
    main(parse_args())
