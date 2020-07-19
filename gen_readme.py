from pathlib import Path
import os
from collections import OrderedDict


def main():
    src_path = Path('src')
    bin_path = src_path / 'bin'

    rust_files = os.listdir(bin_path)

    template = """
# Leetcode Rust

## Problems

| # | Title | Command |
| - | ----- | ------- |
{problems}
    """

    problem_template = "| {number} | {link} | {command} |"

    problems = []
    for rust_file in rust_files:
        # 135-candy.rs -> 135-candy, rs
        filename_and_ext = rust_file.split('.')

        # 135-candy -> 135, candy
        segments = filename_and_ext[0].split('-')

        number = segments[0]
        filename = ' '.join(segments[1:])

        # src/bin/135-candy.rs
        relative_path = '/'.join((bin_path / rust_file).parts)
        link = f'[{filename}]({relative_path})'

        # 135-candy
        command = f'`cargo run --bin {filename_and_ext[0]}`'

        problem = problem_template.format(
            number=number, link=link, command=command)
        problems.append((int(number), problem))
        # problem_dict[int(number)] = problem

    problems.sort(key=lambda x: x[0])

    # readme = template.format(problems='\n'.join(problems))
    readme = template.format(problems='\n'.join(map(lambda x: x[1], problems)))

    # print(readme)

    with open('README.md', 'w') as f:
        f.write(readme)


if __name__ == "__main__":
    main()
