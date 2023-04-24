VERSION --arg-scope-and-set 0.7

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
    ARG --required tag
    ARG tag_suffix
    SAVE IMAGE ${image_name}:${tag}${tag_suffix}

SAVE_IMAGE_PUSH:
    COMMAND
    ARG EARTHLY_GIT_SHORT_HASH
    ARG EARTHLY_TARGET_TAG_DOCKER
    ARG --required tag
    ARG --required image_name
    ARG tag_suffix
    SAVE IMAGE --push ${image_name}:${tag}${tag_suffix}

SAVE_IMAGE_GHCR:
    COMMAND
    ARG --required tag
    ARG tag_suffix
    ARG org_name=andlaz
    ARG repo_name=k8sss
    DO +SAVE_IMAGE_PUSH --image_name="ghcr.io/${org_name}/${repo_name}" --tag=${tag} --tag_suffix=${tag_suffix}

style:
    ARG --required version
    DO +RUST_BUILD_ENV --version ${version}
    RUN rustup component add clippy && \
        rustup component add rustfmt
    RUN cargo fmt --check
    RUN cargo clippy

build:
    DO +RUST_BUILD_ENV
    RUN cargo build --release && chmod +x target/release/k8sss
    ARG style=false
    IF [ "$style" = "true" ]
        BUILD +style
    END
    SAVE ARTIFACT target/release/k8sss /k8sss

libgcc:
    FROM gcr.io/distroless/cc-debian11
    SAVE ARTIFACT /lib/x86_64-linux-gnu/libgcc_s.so.1

image:
    ARG style=false
    ARG base_image=gcr.io/distroless/base-debian11
    FROM ${base_image}
    COPY (+build/k8sss --style $style) /k8sss
    COPY (+libgcc/libgcc_s.so.1) /lib/x86_64-linux-gnu/libgcc_s.so.1

    ENTRYPOINT ["/k8sss"]

    ARG save_cmd="SAVE_IMAGE"
    ARG name="k8sss"
    ARG --required tag
    ARG tag_suffix
    DO .+${save_cmd} --image_name=$name --tag=$tag --tag_suffix=$tag_suffix

baseimage-centos7:
    FROM centos:7
    RUN yum update -y
    # default to a non-root user for centos builds
    USER nobody

chart:
    FROM debian:buster

    DO +BUILD_DEPS

    WORKDIR /k8sss
    COPY --dir charts /k8sss/
    COPY (+version/version) /k8sss/

    RUN cd charts/k8sss && \
        helm package . --version=$(cat ../../version)

    SAVE ARTIFACT /k8sss/charts/k8sss/k8sss*.tgz

test-chart:
    FROM debian:buster

    DO +BUILD_DEPS
    WORKDIR /k8sss
    COPY --dir charts /k8sss/

    RUN cd charts/test && \
        helm dep up && \
        helm --debug template .

    RUN cd charts/test && \
        mkdir out && \
        helm dep up && \
        helm template . \
            --set global.k8sss.image="k8sss:local" \
            --set global.k8sss.imagePullPolicy=Never \
            --set global.k8sss.debug=true \
            --output-dir out && \
        cat out/test/templates/deployment.yaml \
            | yq -e 'select(.spec.template.spec.initContainers[].image == "k8sss:local")' && \
        cat out/test/templates/deployment.yaml \
            | yq -e 'select(.spec.template.spec.initContainers[].imagePullPolicy == "Never")'

version:
    FROM gittools/gitversion:5.12.0-ubuntu.18.04-6.0

    WORKDIR /repo
    COPY --dir .git /repo/
    COPY gitversion.yml /repo/

    DO +BUILD_DEPS

    RUN set -xe; cd /repo && \
        (/tools/dotnet-gitversion || true) && \
        /tools/dotnet-gitversion /config gitversion.yml > version.json && cat version.json && \
        cat version.json | jq -r .LegacySemVer > version

    SAVE ARTIFACT /repo/version
    SAVE ARTIFACT /repo/version.json
