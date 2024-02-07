#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod termination {
    // you can get this ID by running `subkey inspect <your account id>`
    // or docker run -it --pull=always docker.io/parity/subkey:latest inspect <your account id>
    // this value should be the Account ID, without the 0x prefix
    const DEVELOPER: &str = "1c1a05a987cfafff56f393e2ebdf941c7945f9e45e9e8106cf44b3f5467b7326";

    #[ink(storage)]
    pub struct Termination {
        admin: AccountId,
        nominated_admin: Option<AccountId>,
    }

    impl Termination {

        #[ink(constructor)]
        pub fn new() -> Self {
            let caller_account: AccountId = Self::env().caller();
            let caller_account_as_string = hex::encode(caller_account);
            if caller_account_as_string != DEVELOPER {
				//for easy testing we display the keys in the error message.
				panic!("Only the developer can deploy this contract, and '{}' is not '{}'", caller_account_as_string, DEVELOPER);
			}
            Self {
               admin: caller_account,
               nominated_admin: None,
            }
        }

        // this call allows the admin to assign a new admin
        #[ink(message)]
        pub fn assign_admin(&mut self, nominated_admin: AccountId) {
            if self.admin != self.env().caller() {
				panic!("Only the admin can assign a new admin");
			}
			self.nominated_admin = Some(nominated_admin);
		}

        // this call allows the nominated admin to accept the admin role
        #[ink(message)]
        pub fn accept_admin(&mut self) {
            if self.nominated_admin != Some(self.env().caller()) {
                panic!("Only the nominated admin can accept the admin role");
            }
			self.admin = self.nominated_admin.unwrap();
			self.nominated_admin = None;
		}

        // this call allows the admin to terminate the contract
        #[ink(message)]
        pub fn terminate(&mut self) {
	        if self.admin != self.env().caller() {
				panic!("Only the admin can terminate the contract");
			}
	        self.env().terminate_contract(self.env().caller());

        }

        // this call deposits value into the contract
        #[ink(message, payable)]
        pub fn deposit(&mut self) {
			
		}

        // this call withdraws value from the contract to the specified destination
        #[ink(message)]
		pub fn withdraw(&mut self, destination: AccountId, _value: Balance) {
            if self.admin != self.env().caller() {
                panic!("Only the admin can withdraw from the contract");
            }
            self.env().transfer(destination, _value).unwrap();
        }

    }
}