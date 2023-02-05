VERSION 0.6

BUILD_DEPS:
    COMMAND
    RUN apt-get update && \
        apt-get remove -y jq && \
        apt-get install -y bash curl libssl-dev python3-pip && pip3 install yq==2.13.0 && \
        curl -L -o /usr/bin/jq https://github.com/stedolan/jq/releases/download/jq-1.6/jq-linux64 && chmod +x /usr/bin/jq && \
        curl https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash
    RUN jq --version && pip3 show yq

RUST_BUILD_ENV:
    COMMAND
    FROM rust:1-buster

    WORKDIR /k8sss

    COPY --dir src /k8sss/
    COPY Cargo.toml Cargo.lock /k8sss/

    COPY (+version/version) /k8sss/version
    RUN version=$(cat version); \
        echo "Updating version to ${version} in Cargo.toml"; \
        sed -iE "s/^version = \".*\"\$/version = \"${version}\"/g" Cargo.toml;

SAVE_IMAGE:
    COMMAND
    ARG --required image_name
    ARG tag=cache
    SAVE IMAGE ${image_name}:${tag}

SAVE_IMAGE_PUSH:
    COMMAND
    ARG --required image_name
    ARG --required tag
    ARG EARTHLY_GIT_SHORT_HASH
    ARG EARTHLY_TARGET_TAG_DOCKER
    SAVE IMAGE --push ${image_name}:cache
    SAVE IMAGE --push ${image_name}:${tag}
    SAVE IMAGE --push ${image_name}:gith_${EARTHLY_GIT_SHORT_HASH}

SAVE_IMAGE_GHCR:
    COMMAND
    ARG --required tag
    ARG org_name=andlaz
    ARG repo_name=k8sss
    DO +SAVE_IMAGE_PUSH --image_name="ghcr.io/${org_name}/${repo_name}" --tag=${tag}

style:
    ARG --required version
    DO +RUST_BUILD_ENV --version ${version}
    RUN rustup component add clippy && \
        rustup component add rustfmt
    RUN cargo fmt --check
    RUN cargo clippy

build:
    DO +RUST_BUILD_ENV
    RUN cargo build --release
    ARG style=false
    IF [ "$style" = "true" ]
        BUILD +style
    END
    SAVE ARTIFACT target/release/k8sss /k8sss

image:
    ARG style=false
    FROM debian:buster
    COPY (+build/k8sss --style $style) /k8sss

    ENTRYPOINT ["/k8sss"]

    ARG save_cmd="SAVE_IMAGE"
    ARG name="k8sss"
    ARG --required tag
    DO .+${save_cmd} --image_name=$name --tag=$tag

chart:
    FROM debian:buster

    DO +BUILD_DEPS

    WORKDIR /k8sss
    COPY --dir charts /k8sss/
    COPY (+version/version) /k8sss/

    RUN cd charts/k8sss && \
        helm package . --version=$(cat ../../version)

    SAVE ARTIFACT /k8sss/charts/k8sss/k8sss*.tgz

version:
    FROM gittools/gitversion:5.12.0-ubuntu.18.04-6.0

    WORKDIR /repo
    COPY --dir .git /repo/
    COPY gitversion.yml /repo/

    DO +BUILD_DEPS

    RUN cd /repo && \
        /tools/dotnet-gitversion /config gitversion.yml | yq -r .LegacySemVer > version

    SAVE ARTIFACT /repo/version

