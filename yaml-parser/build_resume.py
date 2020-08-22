#!/usr/bin/env python
"""
    Build the Resume described in YAML as a protocol buffer
"""
import argparse
import os

import yaml
from google.protobuf.json_format import ParseDict

from resume_pb2 import Resume


REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '..'))


def parse_args() -> argparse.Namespace:
    """ Parse command line arguments """
    parser = argparse.ArgumentParser()
    parser.add_argument(
        '--input',
        type=str,
        help='Path to YAML file containing resume data',
        metavar='resume_data.yaml',
        default=os.path.join(REPO_ROOT, 'resume_data.yaml')
    )
    parser.add_argument(
        '--save', '-s',
        type=str,
        help='Path to save compiled protobuf message',
        metavar='out.msg',
        default=os.path.join(REPO_ROOT, 'wasm-app', 'src', 'resume.pb')
    )
    args = parser.parse_args()

    if not args.input:
        parser.print_help()
        exit(1)

    return args


def load_yaml(path: str) -> Resume:
    """ Load resume from a yaml file """
    with open(path, 'r') as f:
        data = yaml.safe_load(f)
    return ParseDict(data, Resume())


def validate_tags(resume: Resume):
    """ Verify all tags in experience section are used, warn otherwise """
    exp_tags = set()
    for exp in resume.experience:
        for duty in exp.duty:
            for tag in duty.tags:
                exp_tags.add(tag)

    skill_tags = set()
    for skill in resume.skills:
        for tag in skill.tags:
            skill_tags.add(tag)

    missing = exp_tags - skill_tags
    if missing:
        print("WARNING: not all tags in experience were included in skills.")
        print(f"Missing tags: {missing}")


def main(args: argparse.Namespace):
    resume = load_yaml(args.input)
    validate_tags(resume)

    if args.save:
        print(f'Saving message to {args.save}')
        with open(args.save, 'wb') as f:
            f.write(resume.SerializeToString())


if __name__ == '__main__':
    main(parse_args())
