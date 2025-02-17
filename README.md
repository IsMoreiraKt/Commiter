# Git Committer
This is a simple programme that allows you to make automatic commits to a Git repository on specified dates. It simulates daily commits for a given period of time, helping to keep the repository active.

## Features
- Performs empty commits to a Git repository for a specified date range.
- Allows you to specify how many commits to perform per day.
- Uses multiple threads to parallelise the process and speed up execution.
- Allows you to configure the number of daily commits, the date range and the target repository.

## How to use
1. Clone this repository:
```bash
git clone https://github.com/IsMoreiraKt/Commiter
cd Commiter/commiter
```

2. Compile the project:
```bash
cargo build --release
```

3. Run the application:
```bash
./target/release/commiter
```

4. Enter the parameters: The programme will ask you to enter the following:
    - **Start date:** The start date in YYYY-MM-DD format.
    - **End date:** The end date in YYYY-MM-DD format.
    - **Number of commits per day:** How many commits will be made each day.
    - **Repository path:** The path to the Git repository where the commits will be made.

## Running example:
```bash
Enter start date (YYYY-MM-DD): 2023-01-01
Enter end date (YYYY-MM-DD): 2023-01-05
Enter number of commits per day: 3
Enter repository path: path
```

This will create empty commits for the days 2023-01-01 to 2023-01-05, with 3 commits per day in the specified repository.

## Licence
This project is released under the [GPL Licence](./LICENSE).
