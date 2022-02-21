describe('Insurance NFT', function () {
  let near;
  let contract;
  let accountId;

  beforeAll(async function () {
    console.log('nearConfig', nearConfig);
    near = await nearlib.connect(nearConfig);
    accountId = nearConfig.contractName;
    contract = await near.loadContract(nearConfig.contractName, {
      viewMethods: ['get_owner_address_nft', 'is_insurance_case', 'is_agent',
        'is_expire_insurance', 'get_insurance_data', 'get_hash_image_nft'],
      changeMethods: ['make_new_insurance', 'set_agent'],
      sender: accountId
    });
  });

  describe('insurance', function () {
    it('contract created successfully', async function () {
      const startedContract = await contract.new();
      const endCounter = await contract.get_num();
      expect(startedContract).toBeTruthy()
    });
    // it('can be decremented', async function () {
    //   await contract.increment();
    //   const startCounter = await contract.get_num();
    //   await contract.decrement();
    //   const endCounter = await contract.get_num();
    //   expect(endCounter).toEqual(startCounter - 1);
    // });
    // it('can be reset', async function () {
    //   await contract.increment();
    //   const startCounter = await contract.get_num();
    //   await contract.reset();
    //   const endCounter = await contract.get_num();
    //   expect(endCounter).toEqual(0);
    // });
  });
});