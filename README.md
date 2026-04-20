🛒 Wishlist Solana (CRUD PDA)

Wishlist Solana es un programa descentralizado (Smart Contract) construido en la blockchain de Solana utilizando el framework Anchor.

Este proyecto implementa un sistema CRUD completo que permite a los usuarios gestionar una Lista de Deseos personal, almacenando productos y sus respectivos precios directamente en la blockchain.

🚀 Características Principales
🔐 Almacenamiento Descentralizado

Utiliza PDAs (Program Derived Addresses) para garantizar que cada usuario tenga una única cuenta vinculada a su wallet, manteniendo los datos seguros y aislados.

⚡ Gestión de Memoria Eficiente

Implementa el macro InitSpace de Anchor para precalcular el tamaño de la cuenta en bytes, optimizando el costo de renta en la red.

🔄 CRUD Completo

Permite:

Crear listas
Agregar productos
Editar precios
Consultar información
Eliminar registros
🛡️ Seguridad

Todas las operaciones críticas están protegidas mediante validaciones (require!), asegurando que solo el owner pueda modificar su información.

🛠️ Estructura de Datos (Estado)

El programa maneja su estado mediante dos estructuras principales:

📦 1. Cuenta: Wishlist

Es la cuenta PDA principal que almacena la información general:

owner (Pubkey): Llave pública del creador
nombre_lista (String): Nombre de la lista (máx. 40 caracteres)
productos (Vec): Arreglo dinámico con hasta 15 productos
📦 2. Dato Interno: Producto

Define la estructura de cada elemento dentro de la lista:

nombre (String): Nombre del producto (máx. 30 caracteres)
precio (u64): Precio en unidades enteras (ideal para lamports o centavos)
⚙️ Operaciones del Programa (Instrucciones)
Operación	Función	Descripción
🟢 CREATE (PDA)	inicializar_wishlist	Crea la cuenta base del usuario con una lista vacía
🟢 CREATE (Dato)	agregar_producto	Agrega un nuevo producto a la lista
🔵 READ	ver_wishlist	Muestra los productos en los logs
🟡 UPDATE	editar_producto	Modifica el precio de un producto
🔴 DELETE	eliminar_producto	Elimina un producto de la lista
🧪 Cómo Probar el Proyecto (Solana Playground)

Este proyecto está optimizado para ejecutarse en Solana Playground (SolPG).

🧩 Paso 1: Compilación y Despliegue
Copia el contenido de lib.rs en un proyecto de Anchor dentro de SolPG
Abre la terminal integrada
Ejecuta:
cargo clean
Presiona Build (🔨)
Presiona Deploy (🚀) para desplegar en Devnet
🎯 Objetivo del Proyecto

Este proyecto fue desarrollado con fines educativos para:

Comprender el uso de PDAs
Implementar lógica CRUD en blockchain
Gestionar estado con Anchor
Desarrollar aplicaciones Web3 sobre Solana
📄 Notas Finales
Proyecto enfocado en aprendizaje de desarrollo Web3
Compatible con Devnet para pruebas
Escalable para integración con frontend (React + Wallet Adapter)

Desarrollado para fines educativos y certificación en desarrollo Web3 sobre Solana. 🚀
