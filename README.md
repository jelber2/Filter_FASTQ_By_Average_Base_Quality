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

PHRED Quality =  Percentile
30            = 0th
30            = 1.25th
30            = 2.5th
30            = 5th
30            = 10th
30            = 20th
30            = 25th
30            = 50th
30            = 75th
30            = 100th
1 total reads
1 kept reads at average Q30

@1
\ACTG
+
????
```

For a real use case, one might do

```bash
zcat example.fastq.gz |./target/release/Filter_FASTQ_By_Average_Base_Quality 30 |pigz > Q30.fastq.gz
```

One could run the command initially and then copy and paste the lower cut-off to filter by and replace that value with 30 to filter say by the 5th percentile.
