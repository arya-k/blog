# My Github Paper blog
[![Netlify Status](https://api.netlify.com/api/v1/badges/523dd062-7ef3-4af4-9785-1a267900d235/deploy-status)](https://app.netlify.com/sites/arya-k/deploys)
This will hopefully be the repository of my personal blog/website. This serves a couple purposes:
 1. My first use of Rust in a project, as my own static site compiler. It'll give me complete control over the construction/formatting of the website, and help me maintain my _website ethos_.
 2. Practice writing my ideas in markdown, which should help me express thoughts more clearly, and act as a repository of knowlege and tidbits I've learned over the years.
 3. Introduction of best web practices - how to format a web project structurally, appropriate HTML practices, etc.

## My _website ethos_
 * Static built
 * No third party tracking
 * No foreign dependencies
 * No fancy animations
 * Clear focus on the content

## Project Structure
 * `posts`: blog posts, in the form of markdown files.
 * `assets`: Css, fonts, and other assets for the site.
 * `src`: Static site builder, programmed in Rust
 * `compiled`: The fully compiled website, hosted on Netlify