import argparse
import subprocess

BINARY_NAME = 'vcs'
BINARY_PATH = f'target/debug/{BINARY_NAME}'
REPO_PATH = 'target/vcs_test'

def check_return_code(process):
    code = process.returncode
    output, error = process.communicate()
    cmd = ' '.join(process.args)
    if code:
        print(f"non zero return code: '{cmd}'")
        exit(1)
    if 'warning' in str(error):
        print(f"warning found: '{cmd}'")
        exit(1)
    return output

def execute_command(command, cwd='.'):
    # print(command)
    process = subprocess.Popen(command.split(), stdout=subprocess.PIPE, stderr=subprocess.PIPE, cwd=cwd)
    return check_return_code(process);

def check_cargo_build():
    print('checking build... ', end='')
    execute_command(f'cargo build --bin {BINARY_NAME}')
    print('OK')

def prepare_repo_path():
    print('prepare repo path... ', end='')
    execute_command(f'rm -rf {REPO_PATH}')
    execute_command(f'mkdir {REPO_PATH}')
    print('OK')

def check_contain(output, subs):
    output = str(output)
    for sub in subs:
        if sub not in output:
            print("Error")
            print(f"Output: {output}")
            print(f"not found substring '{sub}' in output")
            exit(1)

def check_init():
    print('checking vcs init... ', end='')
    output = execute_command(f'{BINARY_PATH} init --path {REPO_PATH}')
    subs = [
        'Initialized VCS repository in ',
        'Created commit:',
        '[master ',
        '] Initial commit',
    ]
    check_contain(output, subs)
    print('OK')


def check_status(branch, *, modified=[], added=[]):
    print('checking vcs status... ', end='')
    output = execute_command(f'../../{BINARY_PATH} status', REPO_PATH)
    subs = [
        f'On branch {branch}',
    ]
    if modified or added:
        subs.append('Changes to be commited:')
    else:
        subs.append('No changes to be committed')
    subs.extend([f'modified: {file}' for file in modified])
    subs.extend([f'added: {file}' for file in added])
    check_contain(output, subs)
    print('OK')

def check_commit(branch, message, *, modified=[], added=[]):
    print('checking vcs commit... ', end='')
    output = execute_command(f'../../{BINARY_PATH} commit --message "{message}"', REPO_PATH)
    subs = []
    if modified or added:
        subs.append(f'[{branch} ')
        subs.append(f'] {message}')
    else:
        subs.append('No changes to be committed')
    subs.extend([f'modified: {file}' for file in modified])
    subs.extend([f'added: {file}' for file in added])
    check_contain(output, subs)
    print('OK')

def check_log(commits):
    print('checking vcs log... ', end='')
    output = execute_command(f'../../{BINARY_PATH} log', REPO_PATH)
    lines = output.decode("utf-8").split('\n')
    index = 0
    for commit in commits:
        line = lines[index]
        check_contain(line, ["commit "])
        index += 1
        line = lines[index]
        check_contain(line, [f"Date: "])
        index += 1
        line = lines[index]
        check_contain(line, [f"Message: {commit['message']}"])
        index += 1
        line = lines[index]

        modified = commit['modified']
        added = commit['added']
        if modified or added:
            check_contain(line, [f"Changes:"])
            index += 1
            line = lines[index]
            for modify in modified:
                check_contain(line, [f"modified: {modify}"])
                index += 1
                line = lines[index]
            for add in added:
                check_contain(line, [f"added: {added}"])
                index += 1
                line = lines[index]
        else:
            check_contain(line, [f"No changes"])
            index += 1
            line = lines[index]
        index += 1
    print('OK')


def check_new_branch(branch, current_branch):
    print('checking vcs new_branch... ', end='')
    output = execute_command(f'../../{BINARY_PATH} new_branch --name "{branch}"', REPO_PATH)
    subs = []
    if current_branch == 'master':
        subs.append(f'Created a new branch "{branch}" from master\\\'s commit')
    else:
        subs.append(f"Creating a new branch is possible only when you are in the master branch.")
        subs.append(f"Aborting...")
    check_contain(output, subs)
    print('OK')

def check_jump_branch(branch, *, modified=[], added=[]):
    print('checking vcs jump to branch... ', end='')
    output = execute_command(f'../../{BINARY_PATH} jump --branch {branch}', REPO_PATH)
    subs = []
    if modified or added:
        subs.append(f"error: Your local changes to the following files should be commited or dropped:")
        subs.append(f"Please commit your changes or drop them before you jump.")
        subs.extend(modified)
        subs.extend(added)
        subs.append(f"Aborting...")
    else:
        subs.append(f"Successfully jumped to branch {branch}. Current commit: ")

    check_contain(output, subs)
    print('OK')

def check_merge(branch, *, merge_conficts=[], modified=[], added=[]):
    print('checking vcs merge... ', end='')
    check_jump_branch('master')
    output = execute_command(f'../../{BINARY_PATH} merge --branch "{branch}"', REPO_PATH)
    subs = []
    if merge_conficts:
        subs.append("Merge confilict: file has been changed both in master and branch")
        subs.extend(merge_conficts)
        subs.append("Aborting...")
    else:
        subs.append("Successfully created merge commit:")
        if modified or added:
            subs.append("[master ")
            subs.append(f"] Merged branch {branch}.")
            subs.append(f"{len(modified)} files modified, {len(added)} added")
            subs.extend([f"modified: {modify}" for modify in modified])
            subs.extend([f"added: {add}" for add in added])
        else:
            subs.append("No changes to be committed")
        subs.append(f"Deleted {branch}")

    check_contain(output, subs)
    print('OK')



parser = argparse.ArgumentParser(description='VCS project tester')
args = vars(parser.parse_args())

check_cargo_build()
prepare_repo_path()
check_init()
check_status('master')
check_commit('master', "some_message")

check_log([
    {
        'message': 'Initial commit',
        'modified': [],
        'added': [],
    }
])

check_new_branch("new_branch", "master")
check_new_branch("new_branch2", "new_branch")
check_merge("new_branch")
