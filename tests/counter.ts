import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { expect } from "chai";

describe("counter", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Counter as Program<Counter>;
  const myCounter = anchor.web3.Keypair.generate();

  // Generamos una cuenta de "hacker" que no es el dueño
  const hacker = anchor.web3.Keypair.generate();

  it("Inicializa correctamente", async () => {
    await program.methods
      .initialize()
      .accounts({
        myCounter: myCounter.publicKey,
        user: provider.wallet.publicKey,
      })
      .signers([myCounter])
      .rpc();
  });


    it("Incremento del contador", async () => {
        await program.methods
        .update()
        .accounts({
            myCounter: myCounter.publicKey,
            owner: provider.wallet.publicKey,
        })
        .rpc();

        const account = await program.account.counter.fetch(myCounter.publicKey);
        console.log("Valor del contador despues de la actualizacion", account.count.toString());
    } );

  it("Falla si un extraño intenta incrementar", async () => {
    try {
      await program.methods
        .update()
        .accounts({
          myCounter: myCounter.publicKey,
          owner: hacker.publicKey, // Intentamos usar la llave del hacker
        })
        .signers([hacker]) // El hacker firma la transacción
        .rpc();

      // Si llegamos aquí, el test falló porque el programa permitió al hacker entrar
      expect.fail("El programa debería haber rechazado al hacker");
    } catch (err) {
      // Verificamos que el error sea nuestro ErrorCode::Unauthorized
      expect(err.error.errorCode.code).to.equal("Unauthorized");
      console.log("Mensaje de error capturado:", err.error.errorMessage);
    }
  });
});
