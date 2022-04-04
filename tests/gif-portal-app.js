const anchor = require('@project-serum/anchor');

const { SystemProgram } = anchor.web3;

describe('gif-portal-app', () => {

  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.GifPortalApp;
  const baseAccount = anchor.web3.Keypair.generate();

  it('Is initialized!', async () => {
    const tx = await program.rpc.initialize({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [baseAccount],
    });
    console.log("Your transaction signature", tx);

    let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('👀 GIF Count', account.totalGifsCount.toString());

    await program.rpc.addGif("https://media.giphy.com/media/12HZukMBlutpoQ/giphy.gif", {
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
      },
    });
    account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('👀 GIF Count', account.totalGifsCount.toString());
    console.log('👀 GIF List', account.gifsList);
  });
});
