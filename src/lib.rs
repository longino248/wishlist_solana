use anchor_lang::prelude::*;

// ID del programa
declare_id!("3tQi3CyDbMyYQpK2WnQKmgbo66datgWo4EjKN2UfJvHA");

#[program]
pub mod wishlist_proyect {
    use super::*;

    // 1. CREATE (Cuenta): Inicializa la lista de deseos para el usuario
    pub fn inicializar_wishlist(ctx: Context<CrearWishlist>, nombre_lista: String) -> Result<()> {
        let wishlist = &mut ctx.accounts.wishlist;
        wishlist.owner = ctx.accounts.owner.key();
        wishlist.nombre_lista = nombre_lista;
        wishlist.productos = Vec::new();
        
        msg!("Wishlist '{}' creada.", wishlist.nombre_lista);
        Ok(())
    }

    // 2. CREATE (Dato): Agrega un producto a la lista
    pub fn agregar_producto(ctx: Context<GestionarWishlist>, nombre: String, precio: u64) -> Result<()> {
        let wishlist = &mut ctx.accounts.wishlist;
        
        require!(wishlist.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let nuevo_producto = Producto {
            nombre,
            precio,
        };

        wishlist.productos.push(nuevo_producto);
        msg!("Producto agregado a la lista.");
        Ok(())
    }

    // 3. UPDATE: Cambia el precio de un producto
    pub fn editar_producto(ctx: Context<GestionarWishlist>, nombre: String, nuevo_precio: u64) -> Result<()> {
        let wishlist = &mut ctx.accounts.wishlist;
        require!(wishlist.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let lista = &mut wishlist.productos;
        for i in 0..lista.len() {
            if lista[i].nombre == nombre {
                lista[i].precio = nuevo_precio;
                msg!("Producto '{}' actualizado.", nombre);
                return Ok(());
            }
        }
        Err(Errores::ProductoNoEncontrado.into())
    }

    // 4. DELETE: Elimina un producto de la lista
    pub fn eliminar_producto(ctx: Context<GestionarWishlist>, nombre: String) -> Result<()> {
        let wishlist = &mut ctx.accounts.wishlist;
        require!(wishlist.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let lista = &mut wishlist.productos;
        let index = lista.iter().position(|p| p.nombre == nombre);

        if let Some(i) = index {
            lista.remove(i);
            msg!("Producto '{}' eliminado de la wishlist.", nombre);
            Ok(())
        } else {
            Err(Errores::ProductoNoEncontrado.into())
        }
    }

    // 5. READ: Muestra todos los productos en el log
    pub fn ver_wishlist(ctx: Context<GestionarWishlist>) -> Result<()> {
        msg!("Lista: {}", ctx.accounts.wishlist.nombre_lista);
        msg!("Productos: {:#?}", ctx.accounts.wishlist.productos);
        Ok(())
    }
}

// --- ESTADO DEL PROGRAMA ---

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Producto {
    #[max_len(30)]
    pub nombre: String,
    pub precio: u64, // En unidades enteras
}

#[account]
#[derive(InitSpace)]
pub struct Wishlist {
    pub owner: Pubkey,
    #[max_len(40)]
    pub nombre_lista: String,
    #[max_len(15)] // Capacidad para 15 productos
    pub productos: Vec<Producto>,
}

// --- CONTEXTOS ---

#[derive(Accounts)]
pub struct CrearWishlist<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + Wishlist::INIT_SPACE,
        seeds = [b"wishlist", owner.key().as_ref()],
        bump
    )]
    pub wishlist: Account<'info, Wishlist>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionarWishlist<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub wishlist: Account<'info, Wishlist>,
}

// --- ERRORES ---

#[error_code]
pub enum Errores {
    #[msg("No eres el dueño de esta lista.")]
    NoEresElOwner,
    #[msg("El producto no existe en la lista.")]
    ProductoNoEncontrado,
}
