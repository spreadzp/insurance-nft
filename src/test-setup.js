(function () {
  let params = new URL(window.location).searchParams;
  console.log("🚀 ~ file: test-setup.js ~ line 3 ~ params", params)
  let keyStore = new nearlib.keyStores.InMemoryKeyStore();
  nearConfig.contractName = params.get('contractName') || nearConfig.contractName;
  console.log("🚀 ~ file: test-setup.js ~ line 6 ~ nearConfig.contractName", nearConfig.contractName)
  keyStore.setKey(nearConfig.networkId, nearConfig.contractName, nearlib.KeyPair.fromString(params.get('privateKey')));
  nearConfig.deps = { keyStore };
})();
