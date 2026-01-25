registry := "registry.territoriolinux.es"
user     := "atareao"
name     := `basename ${PWD}`
version  := `vampus show`

version:
    vampus upgrade --patch

list:
    @just --list

dev:
    cd front && pnpm i && pnpm run build && rm -rf ../back/static && mkdir ../back/static && cp -r ./dist/* ../back/static
    cd back && RUST_LOG=debug cargo run

[working-directory("./frontend")]
frontend:
    @pnpm run dev

[working-directory("./backend")]
backend:
    RUST_LOG=debug cargo run

[working-directory("./backend")]
watch:
    RUST_LOG=debug cargo watch -d 60 run


upgrade:
    #!/bin/fish
    vampus upgrade --patch
    set VERSION $(vampus show)
    cd backend
    cargo update
    cd ..
    git commit -am "Upgrade to version $VERSION"
    git tag -a "$VERSION" -m "Version $VERSION"
    # clean old docker images
    docker image list  | grep {{name}} | sort -r | tail -n +5 | awk '{print $3}' | while read id; echo $id; docker rmi $id; end
    just build push

build:
    docker buildx build \
        --tag {{registry}}/{{user}}/{{name}}:{{version}} \
        --tag {{registry}}/{{user}}/{{name}}:latest .

push:
    @docker image push --all-tags {{registry}}/{{user}}/{{name}}

[working-directory("./back")]
revert:
    echo ${PWD}
    @sqlx migrate revert --target-version 0

rf:
    export $(cat backend/.env | xargs) && \
    rainfrog --url ${DATABASE_URL}
