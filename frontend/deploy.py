#!/usr/bin/env python3
import subprocess
import sys
import os

def deploy_to_netlify():
    """Deploy to Netlify using zip upload"""
    print("Deploying to Netlify...")

    # Change to frontend directory
    os.chdir(r'C:\Users\prate\linera\linera-poker\frontend')

    # Create deploy command with automatic site creation
    cmd = [
        'npx', 'netlify-cli', 'deploy',
        '--prod',
        '--dir=dist'
    ]

    # Provide automated inputs
    inputs = [
        '',  # Select first option (Link to existing project)
        '',  # Select first option (Use git remote)
    ]

    try:
        process = subprocess.Popen(
            cmd,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            bufsize=1
        )

        # Send inputs
        for inp in inputs:
            process.stdin.write(inp + '\n')
            process.stdin.flush()

        # Read output
        for line in process.stdout:
            print(line, end='')
            if 'https://' in line.lower() and 'netlify' in line.lower():
                print(f"\n✅ Found URL: {line.strip()}")

        process.wait()
        return process.returncode == 0

    except Exception as e:
        print(f"Error: {e}")
        return False

if __name__ == '__main__':
    success = deploy_to_netlify()
    sys.exit(0 if success else 1)
