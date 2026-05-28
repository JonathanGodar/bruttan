# Bruttan - Bittan but rust(?)

Steps to run:
Check required dependencies in flake.nix :) (or install nix package manger + direnv and you get everything for free :) ) 

- Set DATABASE_URL see flake.nix for example.
- doccker compose up - to start server
- sqlx database reset --force # Creates the tables the rust code needs. ## CODE WILL NOT COMPILE IF THIS IS NOTE DONE 
- cargo run # did you do the step above?

Running the rust code generates the open api specification which can then be compiled to generate typescript types


When in the frontend directory do the following
```bash
npm install --dev --force
npm run generate-api # To update the api types from the rust backend
```
