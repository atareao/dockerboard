user    := "atareao"
name    := `basename ${PWD}`
version := `git tag -l  | tail -n1`
os      := `uname -m`

run:
    cargo run --

watch:
    cargo watch -q -c -w src/ -w assets/ -x run

test:
    #--platform=linux/amd64,linux/arm64/v8,linux/arm/v7 \
    export DOCKER_BUILDKIT=1
    docker buildx build \
        --progress=plain \
        --platform=linux/amd64,linux/arm64/v8 \
        --tag {{user}}/{{name}}:latest \
        --tag  {{user}}/{{name}}:{{version}} \
        --push \
        .

build:
    echo {{version}}
    echo {{name}}
    docker build -t {{user}}/{{name}}:{{version}} \
                 -t {{user}}/{{name}}:latest \
                 .

push:
    docker push --all-tags {{user}}/{{name}}
