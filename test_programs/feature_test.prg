a = 2+3 - 5;
b = a + (
    (2 - a) * 3
    
    ) - 12;
c = a+b;

infinite {
    blah = blah + 2;
    infinite {
        infinite {
            a = 2+3 - 5;
            b    = a + ((2 - a) * 3) - 12;
            c               =           a+                           b;
            break;
        }
        c  =      a+b;
        break;
    }
    c=a+b23;
    break;
}

d = a+b/c;
e = "some string value";
f = "some other string value" ~ e;

fn blah (a, b, c) {
    d = a + b + c;
}