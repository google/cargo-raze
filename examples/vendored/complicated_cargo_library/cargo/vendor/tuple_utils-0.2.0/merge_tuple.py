

def to_type(off):
    return chr(ord('A') + off)

for i in range(2, 17):
    vals = [to_type(i) for i in range(0, i)]
    left = vals[:len(vals)/2]
    right = vals[len(vals)/2:]
    print "impl<%s> Split for (%s) {" % (", ".join(vals), ", ".join(vals))
    print "    type Left = (%s);" % (", ".join(left))
    print "    type Right = (%s);" % (", ".join(right))
    print "    fn split(self) -> (Self::Left, Self::Right) {"
    print "        match self {"
    print "            (%s) => " % (", ".join(x.lower() for x in vals))
    print "                ((%s)," % (", ".join(x.lower() for x in left))
    print "                 (%s))" % (", ".join(x.lower() for x in right))
    print "        }"
    print "    }"
    print "}\n"