//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct Write {
            /// Asset account

    
              
          pub asset: solana_program::pubkey::Pubkey,
                /// The account paying for the storage fees

    
              
          pub payer: solana_program::pubkey::Pubkey,
                /// The system program

    
              
          pub system_program: solana_program::pubkey::Pubkey,
      }

impl Write {
  pub fn instruction(&self, args: WriteInstructionArgs) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(args, &[])
  }
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, args: WriteInstructionArgs, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new(
            self.asset,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.payer,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false
          ));
                      accounts.extend_from_slice(remaining_accounts);
    let mut data = WriteInstructionData::new().try_to_vec().unwrap();
          let mut args = args.try_to_vec().unwrap();
      data.append(&mut args);
    
    solana_program::instruction::Instruction {
      program_id: crate::ASSET_ID,
      accounts,
      data,
    }
  }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct WriteInstructionData {
            discriminator: u8,
                  }

impl WriteInstructionData {
  fn new() -> Self {
    Self {
                        discriminator: 5,
                                              }
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WriteInstructionArgs {
                  pub overwrite: bool,
                pub bytes: Vec<u8>,
      }


/// Instruction builder for `Write`.
///
/// ### Accounts:
///
                      ///   0. `[writable, signer]` asset
                      ///   1. `[writable, signer]` payer
                ///   2. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Default)]
pub struct WriteBuilder {
            asset: Option<solana_program::pubkey::Pubkey>,
                payer: Option<solana_program::pubkey::Pubkey>,
                system_program: Option<solana_program::pubkey::Pubkey>,
                        overwrite: Option<bool>,
                bytes: Option<Vec<u8>>,
        __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl WriteBuilder {
  pub fn new() -> Self {
    Self::default()
  }
            /// Asset account
#[inline(always)]
    pub fn asset(&mut self, asset: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.asset = Some(asset);
                    self
    }
            /// The account paying for the storage fees
#[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.payer = Some(payer);
                    self
    }
            /// `[optional account, default to '11111111111111111111111111111111']`
/// The system program
#[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.system_program = Some(system_program);
                    self
    }
                    #[inline(always)]
      pub fn overwrite(&mut self, overwrite: bool) -> &mut Self {
        self.overwrite = Some(overwrite);
        self
      }
                #[inline(always)]
      pub fn bytes(&mut self, bytes: Vec<u8>) -> &mut Self {
        self.bytes = Some(bytes);
        self
      }
        /// Add an aditional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: solana_program::instruction::AccountMeta) -> &mut Self {
    self.__remaining_accounts.push(account);
    self
  }
  /// Add additional accounts to the instruction.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[solana_program::instruction::AccountMeta]) -> &mut Self {
    self.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[allow(clippy::clone_on_copy)]
  pub fn instruction(&self) -> solana_program::instruction::Instruction {
    let accounts = Write {
                              asset: self.asset.expect("asset is not set"),
                                        payer: self.payer.expect("payer is not set"),
                                        system_program: self.system_program.unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
                      };
          let args = WriteInstructionArgs {
                                                              overwrite: self.overwrite.clone().expect("overwrite is not set"),
                                                                  bytes: self.bytes.clone().expect("bytes is not set"),
                                    };
    
    accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
  }
}

  /// `write` CPI accounts.
  pub struct WriteCpiAccounts<'a, 'b> {
                  /// Asset account

      
                    
              pub asset: &'b solana_program::account_info::AccountInfo<'a>,
                        /// The account paying for the storage fees

      
                    
              pub payer: &'b solana_program::account_info::AccountInfo<'a>,
                        /// The system program

      
                    
              pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
            }

/// `write` CPI instruction.
pub struct WriteCpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
            /// Asset account

    
              
          pub asset: &'b solana_program::account_info::AccountInfo<'a>,
                /// The account paying for the storage fees

    
              
          pub payer: &'b solana_program::account_info::AccountInfo<'a>,
                /// The system program

    
              
          pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
            /// The arguments for the instruction.
    pub __args: WriteInstructionArgs,
  }

impl<'a, 'b> WriteCpi<'a, 'b> {
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
          accounts: WriteCpiAccounts<'a, 'b>,
              args: WriteInstructionArgs,
      ) -> Self {
    Self {
      __program: program,
              asset: accounts.asset,
              payer: accounts.payer,
              system_program: accounts.system_program,
                    __args: args,
          }
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], &[])
  }
  #[inline(always)]
  pub fn invoke_with_remaining_accounts(&self, remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
  }
  #[inline(always)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed_with_remaining_accounts(
    &self,
    signers_seeds: &[&[&[u8]]],
    remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]
  ) -> solana_program::entrypoint::ProgramResult {
    let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new(
            *self.asset.key,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.payer.key,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false
          ));
                      remaining_accounts.iter().for_each(|remaining_account| {
      accounts.push(solana_program::instruction::AccountMeta {
          pubkey: *remaining_account.0.key,
          is_signer: remaining_account.1,
          is_writable: remaining_account.2,
      })
    });
    let mut data = WriteInstructionData::new().try_to_vec().unwrap();
          let mut args = self.__args.try_to_vec().unwrap();
      data.append(&mut args);
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::ASSET_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(3 + 1 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.asset.clone());
                        account_infos.push(self.payer.clone());
                        account_infos.push(self.system_program.clone());
              remaining_accounts.iter().for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

    if signers_seeds.is_empty() {
      solana_program::program::invoke(&instruction, &account_infos)
    } else {
      solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
    }
  }
}

/// Instruction builder for `Write` via CPI.
///
/// ### Accounts:
///
                      ///   0. `[writable, signer]` asset
                      ///   1. `[writable, signer]` payer
          ///   2. `[]` system_program
pub struct WriteCpiBuilder<'a, 'b> {
  instruction: Box<WriteCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> WriteCpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(WriteCpiBuilderInstruction {
      __program: program,
              asset: None,
              payer: None,
              system_program: None,
                                            overwrite: None,
                                bytes: None,
                    __remaining_accounts: Vec::new(),
    });
    Self { instruction }
  }
      /// Asset account
#[inline(always)]
    pub fn asset(&mut self, asset: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.asset = Some(asset);
                    self
    }
      /// The account paying for the storage fees
#[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.payer = Some(payer);
                    self
    }
      /// The system program
#[inline(always)]
    pub fn system_program(&mut self, system_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.system_program = Some(system_program);
                    self
    }
                    #[inline(always)]
      pub fn overwrite(&mut self, overwrite: bool) -> &mut Self {
        self.instruction.overwrite = Some(overwrite);
        self
      }
                #[inline(always)]
      pub fn bytes(&mut self, bytes: Vec<u8>) -> &mut Self {
        self.instruction.bytes = Some(bytes);
        self
      }
        /// Add an additional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: &'b solana_program::account_info::AccountInfo<'a>, is_writable: bool, is_signer: bool) -> &mut Self {
    self.instruction.__remaining_accounts.push((account, is_writable, is_signer));
    self
  }
  /// Add additional accounts to the instruction.
  ///
  /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
  /// and a `bool` indicating whether the account is a signer or not.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> &mut Self {
    self.instruction.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed(&[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
          let args = WriteInstructionArgs {
                                                              overwrite: self.instruction.overwrite.clone().expect("overwrite is not set"),
                                                                  bytes: self.instruction.bytes.clone().expect("bytes is not set"),
                                    };
        let instruction = WriteCpi {
        __program: self.instruction.__program,
                  
          asset: self.instruction.asset.expect("asset is not set"),
                  
          payer: self.instruction.payer.expect("payer is not set"),
                  
          system_program: self.instruction.system_program.expect("system_program is not set"),
                          __args: args,
            };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

struct WriteCpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_program::account_info::AccountInfo<'a>,
            asset: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                        overwrite: Option<bool>,
                bytes: Option<Vec<u8>>,
        /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}

