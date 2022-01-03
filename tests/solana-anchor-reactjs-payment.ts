import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { SolanaAnchorReactjsPayment } from '../target/types/solana_anchor_reactjs_payment';

describe('solana-anchor-reactjs-payment', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SolanaAnchorReactjsPayment as Program<SolanaAnchorReactjsPayment>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
