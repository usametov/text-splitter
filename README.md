## Semantic Text Splitter.

This program processes a group of text files and generates a collection of semantic chunks for each file in JSON format. This project is an extension of the work found at this [GitHub repository](https://github.com/benbrandt/text-splitter), which itself is based on another project documented [here](https://semantic-text-splitter.readthedocs.io/en/stable/semantic_text_splitter.html).

## The Motivation

We want avoid the use of a global constant to determine the size of text chunks. We believe that there must be a more effective method. One potential solution could be the utilization of embeddings to identify clusters of texts that share semantic similarities.

Our underlying assumption is that chunks of text that are semantically similar should be grouped together. This is based on the idea that meaning in language often extends beyond individual sentences, and that by considering larger chunks of text, we can capture more of this meaning.

Furthermore, by grouping similar sentences together, we aim to reduce the amount of noise in the data. Noise, in this context, refers to random or irrelevant information that can interfere with our ability to extract meaningful insights from the text. By reducing this noise, we hope to enhance the clarity and depth of the information we can glean from the text.

In essence, our goal is to leverage the power of embeddings and semantic similarity to create more meaningful and insightful representations of text data. This approach has the potential to significantly improve our ability to understand and interpret large bodies of text. 

This notebook effectively visualizes this concept: 
https://github.com/FullStackRetrieval-com/RetrievalTutorials/blob/main/tutorials/LevelsOfTextSplitting/5_Levels_Of_Text_Splitting.ipynb


Certainly, we'll need to experiment with various values for the parameters of minimum and maximum characters. That's the reason we aim to utilize a Docker image to execute this as an element of our MLOps workflow. 

Here's the command to run this in Docker:
```
docker run -v /home/user777/code/rust/text-splitter/tests:/data <image-name> /target/release/text-splitter --minchar 200 --maxchar 500 --input-files /data/inputs/files2process.txt --dir /data/inputs -o /data

```

To run web server from command line:
```
cargo run -- --minchar 200 --maxchar 500 --input-files nah --dir /data/inputs -o /data -w
```

To build Docker image:
```
docker build . -t <image-name> -f Dockerfile
```

To run web server in Docker:

```
docker run -v /home/user777/code/rust/text-splitter/tests:/data <image-name> /target/release/text-splitter --minchar 200 --maxchar 500 --input-files nah --dir /data/inputs -o /data -w
```

## GCP Cloud deployment steps

Follow these steps to deploy this service to GCP cloud run:

- Create project for our Cloud Run deployment, or reuse an existing one.
- Create Cloud Storage bucket named text-splitter-data.
- Install gcloud tool, run ```gcloud init```. 

Run this in command line:
```
PROJECT=$(gcloud config get-value project)
```
This will set PROJECT environment variable for our GCP project.
We can also set LOCATION environment variable for our GCP project:
```
LOCATION=<type your location here>
```

Run this to list available locations sorted by price:
```
gcloud compute regions list --format="table(name,location,description,available)" --sort-by price
```

Then we need to enable Artifact Registry in our project:
```
gcloud services enable artifactregistry.googleapis.com
```
Now, create repository for your company:
```
gcloud artifacts repositories create --location $LOCATION
--repository-format docker <type-your-company-name>
```
Here is an example:
```
gcloud artifacts repositories create --location us --repository-format docker  example-company
```

We can now construct the image URL in another environment variable:
```
IMAGE=$LOCATION-docker.pkg.dev/$PROJECT/cloud-run-book/hello
```
Finally, you need to build and tag the container image. Make sure you are in the
directory with the source and run docker build:
```
docker build . -t $IMAGE -f Dockerfile
```
You now have the container image with the correct URL, but it’s still stored on your local machine (run ```docker images``` to list all local images).

To let Docker push the local image to Artifact Registry, you need to set up credentials:
```
gcloud auth configure-docker <location>-docker.pkg.dev
```
We can now push the image to Artifact Registry:
```docker push $IMAGE```

And then we deploy it:
```
gcloud run deploy --image $IMAGE --execution-environment gen2 --add-volume=name=data,type=cloud-storage,bucket=text-splitter-data --add-volume-mount=volume=data,mount-path=/data --command /text-splitter --args="-w"
```
Then we navigate to GCP console to get the url of our Cloud Run service and save it in the environment variable:
```
ENDPOINT = <our-service-url>
```

By default, Cloud Run is deployed in private mode by default, this means that unauthenticated user, or authenticated user with the role roles/run.invoker, can’t invoke the service (or an another role with the required permissions).
Therefore, we need to use a project owner account. Follow steps below:
```
gcloud auth list
```
This will list available accounts for our gcloud command, including project owner account. Then run this command to set active account to project owner:
```
gcloud config set account <project_owner_account>
```

Finally, print the token for project owner account:
```
gcloud auth print-identity-token
```
Now we can use that token for api calls.
Before we call our service, we need to upload text files to our bucket.
Our service will process files in this bucket and save it in output folder.

```
curl -X POST \
  $ENDPOINT/api/v1/run \
  -H 'Content-Type: application/json' \
  -d '{"list_of_files": ["example--vesting.mdx", "example--gift-card.mdx"]}'
```

