# Eye of Satoshi Summer of Bitcoin Competence Test 2023

## General Description

This repository contains the files created on completing the competence test of "Generalize rust-teos to support different SQL engines" Eye of Satoshi organization project in Summer of Bitcoin 2023.

Submitted by: Mohamed Awnallah <mohamedmohey2352@gmail.com> <br>
Operating System: Fedora 37 64bit <br>
Rust Compiler Version: rustc 1.67.1 (d5a82bbd2 2023-02-07) (Fedora 1.67.1-1.fc37) <br>
Cargo Package Manager Version: cargo 1.67.1 (8ecd4f20a 2023-01-10) <br>

## Description of the Competence Test
1.
```
Compile rust-teos and run the tower
```
The both images after compiling rust-teos and running the tower could be found respectively using the following links [compiling-rust-teos](assets/compiling-rust-teos), [running-the-tower](assets/running-the-tower).

Compiling and Installing Rust Teos <br>
![compiling-rust-teos](assets/compiling-rust-teos.png)

Running Bitcoind<br>
![running-bitcoind](assets/running-bitcoind.png)

Generate 100 blocks <br>
![generating-100-blocks](assets/generating-100-blocks.png)

Running the tower <br>
![running-the-twoer](assets/running-the-tower.png)

Running the Polar Lightning network with 4 bitcoin cores and 2 Lightning nodes <br>
![building-lightning-network](assets/building-lightning-network.png)

Registering tower in bob lightning node <br>
![registering-tower](assets/registering-tower.png)

Creating invoice from Bob to Alice on Polar Lightning Network <br>
![creating-invoice-from-bob-to-alice](assets/creating-invoice-from-bob-to-alice.png)

Getting approved appointmnets <br>
![getting-approved-appointments](assets/getting-approved-appointments.png)


2.
```
Run the test suite
```

The following image shows that all test cases succeeded after running the following commnad:
```bash
cargo test
```

![running-the-test-suite](assets/running-the-test-suite.png)

3.
```
Create a basic example of loading, storing, and updating data to the teos database
```
The following images for loading, storing, and updating data to the teos database respectively [loading-data-to-teos-database](assets/loading-data-to-teos-database), [storing-data-to-teos-database](assets/storing-data-to-teos-database), and [updating-data-to-teos-database](assets/updatin-data-teos-database)

Loading data to teos database
![loading-data-to-teos-database](assets/loading-data-to-teos-database.png)

Storing data to teos database
![storing-data-to-teos-database](assets/storing-data-to-teos-database.png)

Updating data to teos database
![updating-data-to-teos-database](assets/updating-data-to-teos-database.png)

TODO Add commands you run for this

## Instructions to build and run the simple script for loading, storing, updating data to teos-database:
- Clone this repository and go to the location of the cloned repo.
- Go to the scripts directory:
  ```
  cd scripts
  ```
- Run the bash script using the following command:
```
./scripts.sh
```
#TODO What are the observations

## Additional Resources
This training video "Deploying an Eye of Satoshi Lightning Watchtower" helped me to configure Polar Lightning Network and integrate with watch tower. You could access it [here](https://www.youtube.com/watch?v=8vzNB_NZt2A&t=2194s)
