
KUSTOMIZE=go run sigs.k8s.io/kustomize/kustomize/v5@latest

deploy:
	cd ${DEPLOYMENT} && $(KUSTOMIZE) edit set image textile/basin_worker=${IMAGE_BASIN_WORKER} textile/basin_exporter=${IMAGE_BASIN_EXPORTER}
	cd ${DEPLOYMENT} && $(KUSTOMIZE) build . | kubectl apply -f -
.PHONY: deploy
