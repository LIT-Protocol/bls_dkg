use std::collections::HashSet;
use std::iter::FromIterator;
use xor_name::XorName;

use xor_name::xor_name;

/// ShareXorName is a struct to manage adding and removing XorNames that participate in the DKG.
/// It aims to not reassign shares, and to reuse previously assigned shares, although it does not
/// currently remember names that dropped off so as to try to give them back their old share.
/// There is a lot of possibility for leaking more shares than intended here, so be careful.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShareXorName {
    pub xornames: Vec<XorName>,
    pub shares: Vec<u64>, // really Fr, but for compatibility use u64, or T: IntoFr
    pub available: Vec<u64>, //in decreasing order, so popping gives lowest-value share
}

impl ShareXorName {
    // note that this gives a different assignment of share than inserting
    // the xornames one by one; this is lexicogrphic.
    pub fn from_xornames(xornames: Vec<XorName>) -> ShareXorName {
        // TODO assert all unique
        // TODO assert not too many (less than field max to avoid assigning zero)
        let length = xornames.len();
        let mut xornames = xornames;
        xornames.sort();
        ShareXorName {
            xornames,
            shares: (1..=length).map(|x| x as u64).collect(),
            available: Vec::<u64>::new(),
        }
        // no sort is needed
    }
    fn sort(&mut self) {
        let mut s: Vec<(XorName, u64)> = self
            .xornames
            .clone()
            .into_iter()
            .zip(self.shares.clone())
            .collect();
        s.sort();
        let (x, y): (Vec<_>, Vec<_>) = s.into_iter().unzip();
        self.xornames = x;
        self.shares = y;
    }

    pub fn get_pairs(&self) -> Vec<(XorName, u64)> {
        self.xornames
            .clone()
            .into_iter()
            .zip(self.shares.clone())
            .collect()
    }

    // remove an xorname if present, placing its share in available pool
    fn remove_xorname(&mut self, xorname: XorName) {
        if let Some(position) = self.xornames.iter().position(|&name| name == xorname) {
            let share = self.shares[position];
            self.xornames.remove(position);
            self.shares.remove(position);
            self.available.push(share);
        }
        self.available.sort_by(|a, b| b.cmp(a)); // sort() and reverse()
                                                 // no sort of xornames and shares is needed
    }
    // remove xornames if present, placing shares in available pool
    pub fn remove_xornames(&mut self, rem_xornames: Vec<XorName>) {
        let shares = Vec::<u64>::new();
        for (position, name) in self.xornames.clone().iter().enumerate() {
            if rem_xornames.contains(name) {
                self.available.push(self.shares[position]);
                self.xornames.remove(position);
                self.shares.remove(position);
            }
        }
        self.available.sort_by(|a, b| b.cmp(a)); // sort() and reverse()
                                                 // no sort of xornames and shares is needed
    }
    fn add_xorname(&mut self, xorname: XorName) {
        if let Some(share) = self.available.pop() {
            self.xornames.push(xorname);
            self.shares.push(share);
        } else {
            let share = (self.xornames.len() + 1) as u64; // +1 to take next open share
            self.xornames.push(xorname);
            self.shares.push(share);
        }
        self.sort();
    }

    fn iteradd_xornames(&mut self, add_xornames: Vec<XorName>) {
        for xorname in add_xornames {
            self.add_xorname(xorname)
        }
    }

    pub fn add_xornames(&mut self, add_xornames: Vec<XorName>) {
        let mut next_share = self.xornames.len() as u64;
        for xorname in add_xornames {
            if let Some(share) = self.available.pop() {
                self.xornames.push(xorname);
                self.shares.push(share);
            } else {
                next_share += 1;
                self.xornames.push(xorname);
                self.shares.push(next_share);
            }
        }
        self.sort()
    }

    pub fn to_new_xornames(&mut self, new_xornames: Vec<XorName>) {
        let old: HashSet<XorName> = self.xornames.clone().into_iter().collect();
        let new: HashSet<XorName> = new_xornames.into_iter().collect();
        //	let to_keep = old.intersection(new); // not needed
        let to_remove: Vec<XorName> = old.difference(&new).cloned().collect();
        let to_add: Vec<XorName> = new.difference(&old).cloned().collect();
        self.remove_xornames(to_remove);
        self.add_xornames(to_add);
    }
}

#[cfg(test)]
mod tests {
    use super::ShareXorName;
    use xor_name::xor_name;
    use xor_name::XorName;

    #[test]
    fn test_gen_names() {
        let names: Vec<XorName> = (1..10).map(|i| XorName::random()).collect();
        let name2 = xor_name!(1);
        let sxn = ShareXorName::from_xornames(names);
        //println!("{:?}", sxn);
    }

    // The share assignment keeps assigned shares when possible.  So given an unsorted list of
    // xornames, if they are added all at once using from_xornames, they will be sorted and then
    // shares assigned.  This is the previous behavior of bls_dkg and downstream. Hence the lexicographically
    // first xornames gets share 1, second gets share 2, etc.
    // If the xornames are added one at a time, the shares will be assigned in order of arrival.
    // Hence the chronologically first xorname gets share 1, chronologically second share 2, etc.
    // If the xornames are added one by one but in lexicographic order, the two will agree.

    #[test]
    fn test_inorder() {
        //let names: Vec<XorName> = (1..10).map(|i| XorName::random()).collect();
        let names: Vec<XorName> = (1..5).map(|i| xor_name!(i)).collect();
        let mut sxn2 = ShareXorName::from_xornames(Vec::new());
        let mut sxn3 = ShareXorName::from_xornames(Vec::new());
        sxn2.add_xornames(names.clone());
        sxn3.iteradd_xornames(names.clone());
        println!("{:?}", sxn2);
        assert_eq!(sxn3, sxn2);

        let mut sxn = ShareXorName::from_xornames(names);
        assert_eq!(sxn, sxn2);

        println!("1 {:?}", sxn);
        println!("2 {:?}", sxn2);

        println!("{:?}", sxn);
        sxn.add_xorname(XorName::random()); //name is inserted with new share
        println!("{:?}", sxn);
        sxn.remove_xorname(XorName::random()); //nothing happens unless a collision
        println!("{:?}", sxn);
        sxn.remove_xorname(sxn.xornames[2]); // something removed
        println!("{:?}", sxn);
        sxn.add_xorname(XorName::random()); // something addded, reusing existing share
        println!("{:?}", sxn);
    }

    #[test]
    fn test_not_in_order() {
        //let names: Vec<XorName> = (1..10).map(|i| XorName::random()).collect();
        let names: Vec<XorName> = vec![12, 11, 15, 13, 14]
            .into_iter()
            .map(|i| xor_name!(i))
            .collect();
        let mut sxn2 = ShareXorName::from_xornames(Vec::new());
        let mut sxn3 = ShareXorName::from_xornames(Vec::new());
        sxn2.add_xornames(names.clone());
        sxn3.iteradd_xornames(names.clone());
        println!("{:?}", sxn2);
        assert_eq!(sxn3, sxn2);

        let mut sxn = ShareXorName::from_xornames(names);
        assert_ne!(sxn, sxn2);

        println!("1 {:?}", sxn);
        println!("2 {:?}", sxn2);

        println!("{:?}", sxn);
        sxn.add_xorname(XorName::random()); //name is inserted with new share
        println!("{:?}", sxn);
        sxn.remove_xorname(XorName::random()); //nothing happens unless a collision
        println!("{:?}", sxn);
        sxn.remove_xorname(sxn.xornames[2]); // something removed
        println!("{:?}", sxn);
        sxn.add_xorname(XorName::random()); // something addded, reusing existing share
        println!("{:?}", sxn);
    }
}
