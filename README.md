# Node Exporter

Node Exporter is a powerful monitoring tool written in Rust. It collects various system metrics and exposes them to scrape.

## MongoDB Setup in Docker

To set up MongoDB in Docker on port 27017, follow these steps:

1. Install Docker on your machine if you haven't already.
2. Open a terminal and run the following command to pull the MongoDB Docker image:

```bash
docker pull mongo
```

3. Once the image is downloaded, run the following command to start a MongoDB container:

```bash
docker run -d -p 27017:27017 --name mongodb mongo
```

4. MongoDB will now be running in a Docker container, accessible on port 27017.


## Installation

To install Node Exporter, follow these steps:

1. Download the latest release from this repository.
2. Extract the downloaded archive to a directory of your choice.
3. Open a terminal and navigate to the extracted directory.
4. Run the following command to start Node Exporter:

```bash
cargo build 
cargo run
```

5. Node Exporter will now be running.

## To see data

1. Install the required dependencies by running the following command:

```bash
pip install Flask pymongo
```

2. Start the Flask application by running the following command:

```bash
python app.py
```

3. Open your web browser and navigate to `http://localhost:5000` to access the Flask application.

4. If the Flask application is working fine, you should see the json data.


