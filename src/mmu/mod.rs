use crate::bus::Bus;
use crate::cpu::types::PrivilegeMode;
use crate::cpu::Address;

pub enum AccessType {
    Fetch,
    Load,
    Store,
}

// RISC-V Exception Codes for Page Faults
pub const EXC_INST_PAGE_FAULT: u32 = 12;
pub const EXC_LOAD_PAGE_FAULT: u32 = 13;
pub const EXC_STORE_PAGE_FAULT: u32 = 15;

pub struct MMU;

impl MMU {
    pub fn translate(
        vaddr: u32,
        access: &AccessType,
        satp: u32,
        privilege: PrivilegeMode,
        bus: &mut Bus,
    ) -> Result<u32, u32> {
        let mode = (satp >> 31) & 0x1;

        // If MMU is off (mode == 0) or we are in Machine mode, physical == virtual
        if mode == 0 || privilege == PrivilegeMode::Machine {
            return Ok(vaddr);
        }

        // Slice up the Virtual Address
        let vpn1 = (vaddr >> 22) & 0x3FF;
        let vpn0 = (vaddr >> 12) & 0x3FF;
        let offset = vaddr & 0xFFF;

        // Step 2: Read Root Page Table Entry
        let root_ppn = satp & 0x3FFFFF;
        let root_pte_addr = (root_ppn * 4096) + (vpn1 * 4);
        let root_pte = bus.read32(Address(root_pte_addr));

        let v = root_pte & 0x1;
        let r = (root_pte >> 1) & 0x1;
        let w = (root_pte >> 2) & 0x1;
        let x = (root_pte >> 3) & 0x1;

        // Check Valid bit, and invalid R/W combinations
        if v == 0 || (r == 0 && w == 1) {
            return Err(Self::fault_cause(access));
        }

        // We will store the final parsed PTE here
        let mut leaf_pte = root_pte;

        // Step 3: Is it a pointer to a Leaf Table, or a Mega Page?
        if r == 0 && w == 0 && x == 0 {
            // It's a pointer to the next level (Leaf Table)
            let leaf_ppn = (root_pte >> 10) & 0x3FFFFF;
            let leaf_pte_addr = (leaf_ppn * 4096) + (vpn0 * 4);
            leaf_pte = bus.read32(Address(leaf_pte_addr));

            let lv = leaf_pte & 0x1;
            let lr = (leaf_pte >> 1) & 0x1;
            let lw = (leaf_pte >> 2) & 0x1;
            let lx = (leaf_pte >> 3) & 0x1;

            if lv == 0 || (lr == 0 && lw == 1) {
                return Err(Self::fault_cause(access));
            }

            if lr == 0 && lw == 0 && lx == 0 {
                // Sv32 only supports 2 levels. A pointer at the leaf level is illegal.
                return Err(Self::fault_cause(access));
            }
        } else {
            // In RISC-V, mega pages require the VPN[0] of the virtual address to be 0
            if vpn0 != 0 {
                return Err(Self::fault_cause(access));
            }
        }

        // Step 4: Check Permissions against the active Privilege Mode
        let leaf_r = (leaf_pte >> 1) & 0x1;
        let leaf_w = (leaf_pte >> 2) & 0x1;
        let leaf_x = (leaf_pte >> 3) & 0x1;
        let leaf_u = (leaf_pte >> 4) & 0x1;

        // User mode can't access non-user memory
        if privilege == PrivilegeMode::User && leaf_u == 0 {
            return Err(Self::fault_cause(access));
        }
        
        // Supervisor mode generally cannot access User memory unless a special flag (SUM) 
        // is set in the mstatus register. For a basic MMU, we enforce strict separation.
        if privilege == PrivilegeMode::Supervisor && leaf_u == 1 {
            return Err(Self::fault_cause(access));
        }

        // Check specific access type permissions
        match access {
            AccessType::Fetch => {
                if leaf_x == 0 {
                    return Err(Self::fault_cause(access));
                }
            }
            AccessType::Load => {
                if leaf_r == 0 {
                    return Err(Self::fault_cause(access));
                }
            }
            AccessType::Store => {
                if leaf_w == 0 {
                    return Err(Self::fault_cause(access));
                }
            }
        }

        // Step 5: Everything passed! Construct the Physical Address
        let final_ppn = (leaf_pte >> 10) & 0x3FFFFF;
        let paddr = (final_ppn * 4096) + offset;

        Ok(paddr)
    }

    fn fault_cause(access: &AccessType) -> u32 {
        match access {
            AccessType::Fetch => EXC_INST_PAGE_FAULT,
            AccessType::Load => EXC_LOAD_PAGE_FAULT,
            AccessType::Store => EXC_STORE_PAGE_FAULT,
        }
    }
}