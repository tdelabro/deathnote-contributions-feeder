use proc_macro::TokenStream;
use quote::quote;

pub fn impl_hex_string_wrapper_macro(ast: &syn::DeriveInput) -> TokenStream {
	let name = &ast.ident;
	let gen = quote! {
		impl Into<U256> for #name {
			fn into(self) -> U256 {
				self.0.into()
			}
		}

		impl FromStr for #name {
			type Err = ParseHexPrefixedStringError;

			fn from_str(s: &str) -> Result<Self, Self::Err> {
				HexPrefixedString::from_str(s).map(Self)
			}
		}

		impl From<U256> for #name {
			fn from(v: U256) -> Self {
				Self(HexPrefixedString::from(v))
			}
		}

		impl Display for #name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "{}", self.0.to_string())
			}
		}

		impl From<u128> for #name {
			fn from(id: u128) -> Self {
				U256::from_u128(id).into()
			}
		}
	};
	gen.into()
}