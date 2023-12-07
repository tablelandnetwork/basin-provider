
KUSTOMIZE=go run sigs.k8s.io/kustomize/kustomize/v5@latest

deploy:
	cd ${DEPLOYMENT} && $(KUSTOMIZE) edit set image textile/basin_worker=${IMAGE_BASIN_WORKER}
	cd ${DEPLOYMENT} && $(KUSTOMIZE) build . | sed -e 's/\x27"3000"\x27/3000/g' | sed -e 's/\x27"443"\x27/443/g' | kubectl apply -f -
.PHONY: deploy
