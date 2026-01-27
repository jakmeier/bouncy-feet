FROM quay.io/keycloak/keycloak:latest as builder

ENV KC_HEALTH_ENABLED=true
ENV KC_HTTP_ENABLED=true
ENV KC_PROXY_HEADERS=xforwarded
ENV KC_DB=postgres

WORKDIR /opt/keycloak
RUN /opt/keycloak/bin/kc.sh build

FROM quay.io/keycloak/keycloak:latest
COPY --from=builder /opt/keycloak/ /opt/keycloak/

ENV KC_HTTP_ENABLED=true

# Run with disabled cache for easy theme development
ENTRYPOINT ["/opt/keycloak/bin/kc.sh", "start", "--optimized", "--spi-theme--static-max-age=-1", "--spi-theme--cache-themes=false", "--spi-theme--cache-templates=false"]
