// Here we export some useful types and functions for interacting with the Anchor program.
import CrudIDL from '../target/idl/crud.json'

// Re-export the generated IDL and type
export { CrudIDL }

export * from './client/js'
