# Filter_FASTQ_By_Average_Base_Quality

## Installation

    cd to-some-desired-base-directory
    git clone https://github.com/jelber2/Filter_FASTQ_By_Average_Base_Quality.git
    cd Filter_FASTQ_By_Average_Base_Quality
    cargo build --release
    
## Usage

Reads in from stdin, allows user to specify average base quality value to filter reads by, and outputs to stdout, other information is outputted to stderr:
For example:

```bash
echo -e "@1\n\ACTG\n+\n????" |./target/release/Filter_FASTQ_By_Average_Base_Quality 30


Q30 = 0th percentile
Q30 = 1.25th percentile
Q30 = 2.5th percentile
Q30 = 5th percentile
Q30 = 10th percentile
Q30 = 20th percentile
Q30 = 25th percentile
Q30 = 50th percentile
Q30 = 75th percentile
Q30 = 100th percentile
1 total reads
1 kept reads at average Q30
@1
\ACTG
+
????
```
