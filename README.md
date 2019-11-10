# dailyprogrammer

## Contributing

### Project structure
Code of a specific programming language should exist in a directory indicating the language (e.g. python_files, rust_files)
New challenges from https://www.reddit.com/r/dailyprogrammer/ should exist in a folder under the language folder
Recommended structure is "project/challenge" / "<project_num>-description" /*

Directory structures should be formatted in a method compliant with the module system of the language written.

Coding challenges outside of the recommended subreddit are encouraged.

All new code should contain a readme.txt file with a description of the challenge. 

Links to source material are recommended at the top line.

## Branching
All changes should exist on a feature branch. Do not commit straight to master. 

## Unit Testing
All changes should be unit tested. 
All python unit tests should follow the naming convention *\*_test.py* so they will be picked up by *dailyprogrammer/test.py*

## Qualifying Contributions
A passing build in automation is a prerequisite for a merge. 

Builds are run automatically every four hours when the server is up.

New code is expected to be tested. New tests are expected to be executed within the Jenkinsfile.

## Conventions
Python code should be formatted according to the PEP 8 standard.

Rust code should compile without warning with *cargo test*
