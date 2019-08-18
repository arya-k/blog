#!/bin/sh

# Arya Kumar
# 08/17/29
# Code heavily based on https://github.com/khanhicetea/gh-actions-hugo-deploy-gh-pages

echo '=================== Create deploy key to push ==================='
mkdir /root/.ssh
ssh-keyscan -t rsa github.com > /root/.ssh/known_hosts && \
echo "${GIT_DEPLOY_KEY}" > /root/.ssh/id_rsa && \
chmod 400 /root/.ssh/id_rsa

echo '=================== Publish to Github ==================='
cd dist
remote_repo="git@github.com:${GITHUB_REPOSITORY}.git"
git init && \
git remote add deploy $remote_repo && 
git config user.name "${GITHUB_ACTOR}" && \
git config user.email "${GITHUB_ACTOR}@users.noreply.github.com" && \
git add . && \
echo -n 'Files to Commit:' && ls -l | wc -l && \
timestamp=$(date +%s%3N) && \
git commit -m "Automated deployment to GitHub Pages on $timestamp" > /dev/null 2>&1 && \
git push deploy --force > /dev/null 2>&1 && \
rm -rf .git && \
cd ../
echo '=================== Done  ==================='