Wishlist Solana (CRUD PDA) 🛒

Wishlist Solana es un programa descentralizado (Smart Contract) construido en la blockchain de Solana utilizando el framework Anchor. Este proyecto implementa un sistema CRUD completo que permite a los usuarios gestionar una "Lista de Deseos" personal, almacenando los productos que desean comprar y sus respectivos precios.
🚀 Características Principales

    Almacenamiento Descentralizado: Utiliza PDAs (Program Derived Addresses) para garantizar que cada usuario tenga una única cuenta vinculada a su llave pública (Wallet), manteniendo sus datos seguros y separados de otros usuarios.

    Gestión de Memoria Eficiente: Implementa el macro InitSpace de Anchor para precalcular el tamaño de la cuenta en bytes, optimizando el pago de "renta" en la red de Solana.

    CRUD Completo: Permite crear listas, agregar productos, editar precios, leer el estado actual y eliminar registros de forma dinámica.

    Seguridad: Todas las funciones de modificación y borrado están protegidas, verificando mediante validaciones (require!) que solo el propietario original (owner) pueda alterar su lista.

🛠️ Estructura de Datos (Estado)

El programa maneja el estado a través de dos estructuras optimizadas:
1. La Cuenta (Wishlist)

Es la cuenta PDA principal que guarda la información general y el contenedor de los datos.

    owner (Pubkey): La llave pública del creador.

    nombre_lista (String): El título de la lista (Max. 40 caracteres).

    productos (Vector): Un arreglo dinámico que soporta hasta 15 productos.

2. El Dato Interno (Producto)

El struct que define qué se guarda por cada ítem.

    nombre (String): Nombre del producto (Max. 30 caracteres).

    precio (u64): Precio del producto (Almacenado como número entero, ideal para representar Lamports o centavos).

⚙️ Operaciones del Programa (Instrucciones)
Operación	Función	Descripción
CREATE (PDA)	inicializar_wishlist	Deriva la PDA del usuario y crea la cuenta base con un vector vacío.
CREATE (Dato)	agregar_producto	Añade un nuevo Producto (Nombre y Precio) al final de la lista.
READ	ver_wishlist	Imprime en los logs de la blockchain el contenido íntegro del vector.
UPDATE	editar_producto	Busca un producto por su nombre y actualiza su precio.
DELETE	eliminar_producto	Busca un producto por su nombre y lo remueve del vector, liberando espacio.
🧪 Cómo Probar el Proyecto (Solana Playground)

Este proyecto está optimizado para ser desplegado y testeado rápidamente en Solana Playground (SolPG).
Paso 1: Compilación y Despliegue

    Copia el contenido de lib.rs en un nuevo proyecto nativo de Anchor en SolPG.

    Abre la terminal integrada y ejecuta cargo clean para evitar problemas de caché con el IDL.

    Presiona el botón Build (Icono de Martillo).

    Presiona el botón Deploy (Icono de Cohete) para subirlo a la Devnet.

Paso 2: Script de Pruebas (TypeScript)

Para verificar la lógica de negocio, dirígete a la carpeta tests, abre el archivo anchor.test.ts y ejecuta el siguiente flujo automatizado de pruebas usando el botón Test:
TypeScript

import * as anchor from "@coral-xyz/anchor";

describe("Test de Wishlist_Solana", () => {
  const [wishlistPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("wishlist"), pg.wallet.publicKey.toBuffer()],
    pg.program.programId
  );

  it("1. Inicializa la lista", async () => {
    const tx = await pg.program.methods
      .inicializarWishlist("Mi Lista Tech")
      .accounts({
        wishlist: wishlistPda,
        owner: pg.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("   ✅ Lista creada:", tx);
  });

  it("2. Agrega un producto", async () => {
    const tx = await pg.program.methods
      .agregarProducto("Laptop", new anchor.BN(1200))
      .accounts({
        wishlist: wishlistPda,
        owner: pg.wallet.publicKey,
      })
      .rpc();
    console.log("   ✅ Producto agregado:", tx);
  });

  it("3. Edita el precio", async () => {
    const tx = await pg.program.methods
      .editarProducto("Laptop", new anchor.BN(1100))
      .accounts({
        wishlist: wishlistPda,
        owner: pg.wallet.publicKey,
      })
      .rpc();
    console.log("   ✅ Precio actualizado:", tx);
  });

  it("4. Visualiza la lista", async () => {
    const cuenta = await pg.program.account.wishlist.fetch(wishlistPda);
    console.log("   📖 Productos actuales:", cuenta.productos);
  });

  it("5. Elimina el producto", async () => {
    const tx = await pg.program.methods
      .eliminarProducto("Laptop")
      .accounts({
        wishlist: wishlistPda,
        owner: pg.wallet.publicKey,
      })
      .rpc();
    console.log("   ✅ Producto eliminado:", tx);
  });
});

Desarrollado para fines educativos y certificación en desarrollo Web3 sobre Solana.
