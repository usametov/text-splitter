Semantic Text Splitter.

This program processes a group of text files and generates a collection of semantic chunks for each file in JSON format. This project is an extension of the work found at this [GitHub repository](https://github.com/benbrandt/text-splitter), which itself is based on another project documented [here](https://semantic-text-splitter.readthedocs.io/en/stable/semantic_text_splitter.html).

## The Motivation

We want avoid the use of a global constant to determine the size of text chunks. We believe that there must be a more effective method. One potential solution could be the utilization of embeddings to identify clusters of texts that share semantic similarities.

Our underlying assumption is that chunks of text that are semantically similar should be grouped together. This is based on the idea that meaning in language often extends beyond individual sentences, and that by considering larger chunks of text, we can capture more of this meaning.

Furthermore, by grouping similar sentences together, we aim to reduce the amount of noise in the data. Noise, in this context, refers to random or irrelevant information that can interfere with our ability to extract meaningful insights from the text. By reducing this noise, we hope to enhance the clarity and depth of the information we can glean from the text.

In essence, our goal is to leverage the power of embeddings and semantic similarity to create more meaningful and insightful representations of text data. This approach has the potential to significantly improve our ability to understand and interpret large bodies of text. 

This notebook effectively visualizes this concept: 
https://github.com/FullStackRetrieval-com/RetrievalTutorials/blob/main/tutorials/LevelsOfTextSplitting/5_Levels_Of_Text_Splitting.ipynb




How to run this in docker:

```
docker run -v ~/Documents/data:/data -it text-splitter
```
