use crate::keypoints::Cartesian3d;
use crate::Keypoints;

impl Keypoints {
    #[allow(dead_code)]
    pub(crate) fn iter(&self) -> KeypointsIter {
        KeypointsIter { i: 0, kp: self }
    }
    
    #[allow(dead_code)]
    pub(crate) fn iter_mut(&mut self) -> KeypointsIterMut {
        // note: It would be nice to avoid extra allocations but it's not that
        // easy to avoid shared &mut. Best I can think of is
        // [Option<Cartesian>;16] but I don't like the extra code complexity.
        KeypointsIterMut {
            kp: vec![
                &mut self.left.ankle,
                &mut self.left.elbow,
                &mut self.left.heel,
                &mut self.left.hip,
                &mut self.left.knee,
                &mut self.left.shoulder,
                &mut self.left.toes,
                &mut self.left.wrist,
                &mut self.right.ankle,
                &mut self.right.elbow,
                &mut self.right.heel,
                &mut self.right.hip,
                &mut self.right.knee,
                &mut self.right.shoulder,
                &mut self.right.toes,
                &mut self.right.wrist,
            ],
        }
    }
}

pub(crate) struct KeypointsIter<'a> {
    i: usize,
    kp: &'a Keypoints,
}

impl Iterator for KeypointsIter<'_> {
    type Item = Cartesian3d;

    fn next(&mut self) -> Option<Self::Item> {
        let out = match self.i {
            0 => self.kp.left.ankle,
            1 => self.kp.left.elbow,
            2 => self.kp.left.heel,
            3 => self.kp.left.hip,
            4 => self.kp.left.knee,
            5 => self.kp.left.shoulder,
            6 => self.kp.left.toes,
            7 => self.kp.left.wrist,
            8 => self.kp.right.ankle,
            9 => self.kp.right.elbow,
            10 => self.kp.right.heel,
            11 => self.kp.right.hip,
            12 => self.kp.right.knee,
            13 => self.kp.right.shoulder,
            14 => self.kp.right.toes,
            15 => self.kp.right.wrist,
            _ => return None,
        };
        self.i += 1;
        Some(out)
    }
}

pub(crate) struct KeypointsIterMut<'a> {
    kp: Vec<&'a mut Cartesian3d>,
}

impl<'a> Iterator for KeypointsIterMut<'a> {
    type Item = &'a mut Cartesian3d;

    fn next(&mut self) -> Option<Self::Item> {
        self.kp.pop()
    }
}
