#!/bin/bash

if [ -z "$1" ]; then
  echo "You must provide a project name"
  exit 1
fi

PROJECT_NAME=$1

mkdir -p $PROJECT_NAME

cp -r template/* $PROJECT_NAME

if [[ "$OSTYPE" == "darwin"* ]]; then
  sed -i '' "s/{{ name }}/$PROJECT_NAME/g" $PROJECT_NAME/package.json
  sed -i '' "s/{{ name }}/$PROJECT_NAME/g" $PROJECT_NAME/src/index.ts
  sed -i '' "s/{{ name }}/$PROJECT_NAME/g" $PROJECT_NAME/test/{{name}}.test.ts
else
  sed -i "s/{{ name }}/$PROJECT_NAME/g" $PROJECT_NAME/package.json
  sed -i "s/{{ name }}/$PROJECT_NAME/g" $PROJECT_NAME/src/index.ts
  sed -i "s/{{ name }}/$PROJECT_NAME/g" $PROJECT_NAME/test/{{name}}.test.ts
fi

mv $PROJECT_NAME/test/{{name}}.test.ts $PROJECT_NAME/test/$PROJECT_NAME.test.ts

cd $PROJECT_NAME
npm install
