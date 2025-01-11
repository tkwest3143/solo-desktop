include .env

dev: 
	yarn tauri dev

gen-entity: 
	source .env && cd ./src-tauri && sea-orm-cli generate entity -u ${DATABASE_URL} -o src/entities && cd ../../

migrate: 
	@echo "Running step 1"
	source .env && cd ./src-tauri && sea-orm-cli migrate refresh && cd ../../