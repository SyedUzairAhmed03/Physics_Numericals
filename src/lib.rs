use std::io;
#[warn(dead_code)]
 pub fn First_equation_of_motion () {
    loop {
        let mut x = String::new();
       
       println!("vf = vi + at");
       println!("choose following number");
       println!("plz press 1 for find : vf");       
       println!("plz press 2 for find : vi");
       println!("plz press 3 for find : a");
       println!("plz press 4 for find : t");
    io::stdin().read_line(&mut x);
       let  x:i8 = x.trim().parse().unwrap();
       if x == 1 {

           let mut g = String::new();
           println!("the value of vi is in ?
press 1 for :m/sec
press 2 for :km/hour");
           io::stdin().read_line(&mut g);
           let  g:i8 = g.trim().parse().unwrap();


          if g == 1{ println!("enter the value of vi");
           let mut vi = String::new();
           io::stdin().read_line(&mut vi);
           let  vi:f64 = vi.trim().parse().unwrap();
              
           let mut a = String::new();
           println!("enter the vaue of a");
           io::stdin().read_line(&mut a);
           let  a:f64 = a.trim().parse().unwrap();

           let mut t = String::new();
           println!("enter the value of t");
           io::stdin().read_line(&mut t);
           let  t:f64 = t.trim().parse().unwrap();
           println!("vf={}m/sec",vi+a*t);
          }else if g == 2{
              println!("enter the value of vi");
           let mut vi = String::new();
           io::stdin().read_line(&mut vi);
           let  vi:f64 = vi.trim().parse().unwrap();

           let d:f64 = vi*1000.00;
           let d=d/3600.00;
              
           let mut a = String::new();
           println!("enter the vaue of a");
           io::stdin().read_line(&mut a);
           let  a:f64 = a.trim().parse().unwrap();

           let mut t = String::new();
           println!("enter the value of t");
           io::stdin().read_line(&mut t);
           let  t:f64 = t.trim().parse().unwrap();
           println!("vf={}m/sec",d+a*t);
          }else{
             println!("Err");
          }
        }else if x == 2{
              let mut g = String::new();
           println!("the value of vf is in ?
press 1 for :m/sec
press 2 for :km/hour");
           io::stdin().read_line(&mut g);
           let  g:i8 = g.trim().parse().unwrap();

      
          if g == 1{ println!("enter the value of vf");
           let mut vf = String::new();
           io::stdin().read_line(&mut vf);
           let  vf:f64 = vf.trim().parse().unwrap();
              
           let mut a = String::new();
           println!("enter the vaue of a");
           io::stdin().read_line(&mut a);
           let  a:f64 = a.trim().parse().unwrap();

           let mut t = String::new();
           println!("enter the value of t");
           io::stdin().read_line(&mut t);
           let  t:f64 = t.trim().parse().unwrap();
           println!("vi={}m/sec",vf-a*t);
          }else if g == 2{
              println!("enter the value of vf");
           let mut vf = String::new();
           io::stdin().read_line(&mut vf);
           let  vf:f64 = vf.trim().parse().unwrap();

           let d:f64 = vf*1000.00;
           let d=d/3600.00;
              
           let mut a = String::new();
           println!("enter the vaue of a"); println!("enter the value of vi");
           let mut vi = String::new();
           io::stdin().read_line(&mut vi);
           let  vi:f64 = vi.trim().parse().unwrap();

             let mut s = String::new();
           println!("enter the vaue of s or h");
           io::stdin().read_line(&mut s);
           let  s:f64 = s.trim().parse().unwrap();

           let mut t = String::new();
           println!("enter the value of t");
           io::stdin().read_line(&mut t);
           let  t:f64 = t.trim().parse().unwrap();

           let d = t*t;
           let j = d/2.0;

           println!("a = {}",(s-(vi*t))/j);
           io::stdin().read_line(&mut a);
           let  a:f64 = a.trim().parse().unwrap();

           let mut t = String::new();
           println!("enter the value of t");
           io::stdin().read_line(&mut t);
           let  t:f64 = t.trim().parse().unwrap();
           println!("vi={}m/sec",d-a*t);
          }else{
             println!("Err");
          }
    }else if x == 3 {
        let mut g = String::new();
           println!("the value of vi and vf is in ?
press 1 for :m/sec
press 2 for :km/hour");
           io::stdin().read_line(&mut g);
           let  g:i8 = g.trim().parse().unwrap();

           if g == 1{
               println!("enter the value of vi");
           let mut vi = String::new();
           io::stdin().read_line(&mut vi);
           let  vi:f64 = vi.trim().parse().unwrap();

           println!("enter the value of vf");
           let mut vf = String::new();
           io::stdin().read_line(&mut vf);
           let  vf:f64 = vf.trim().parse().unwrap();

           let mut t = String::new();
           println!("enter the value of t in sec");
           io::stdin().read_line(&mut t);
           let  t:f64 = t.trim().parse().unwrap();
           println!("a = {}m/sec^2",(vf-vi)/(t));
           }else if g == 2{
              println!("enter the value of vi in km/h");
           let mut vi = String::new();
           io::stdin().read_line(&mut vi);
           let  vi:f64 = vi.trim().parse().unwrap();

           let d:f64 = vi*1000.00;
           let d=d/3600.00;

         println!("enter the value of vf in km/h");
           let mut vf = String::new();
           io::stdin().read_line(&mut vf);
           let  vf:f64 = vf.trim().parse().unwrap();

           let k:f64 = vf*1000.00;
           let k = k/3600.00;

           let mut t = String::new();
           println!("enter the value of t in sec");
           io::stdin().read_line(&mut t);
           let  t:f64 = t.trim().parse().unwrap();
           println!("a = {}m/sec^2",(k-d)/(t));

           }else{
              println!("Err");
           }

    }else if x == 4 {
                 let mut g = String::new();
           println!("the value of vi and vf is in ?
press 1 for :m/sec
press 2 for :km/hour");
           io::stdin().read_line(&mut g);
           let  g:i8 = g.trim().parse().unwrap();

           if g == 1{
               println!("enter the value of vi");
           let mut vi = String::new();
           io::stdin().read_line(&mut vi);
           let  vi:f64 = vi.trim().parse().unwrap();

           println!("enter the value of vf");
           let mut vf = String::new();
           io::stdin().read_line(&mut vf);
           let  vf:f64 = vf.trim().parse().unwrap();

           let mut a = String::new();
           println!("enter the value of a ");
           io::stdin().read_line(&mut a);
           let  a:f64 = a.trim().parse().unwrap();
           println!("t = {} sec",(vf-vi)/(a));
           }else if g == 2{
              println!("enter the value of vi in km/h");
           let mut vi = String::new();
           io::stdin().read_line(&mut vi);
           let  vi:f64 = vi.trim().parse().unwrap();

           let d:f64 = vi*1000.00;
           let d = d/3600.00;

         println!("enter the value of vf in km/h");
           let mut vf = String::new();
           io::stdin().read_line(&mut vf);
           let  vf:f64 = vf.trim().parse().unwrap();

           let k:f64 = vf*1000.00;
           let k = k/3600.00;

           let mut a = String::new();
           println!("enter the value of a in sec");
           io::stdin().read_line(&mut a);
           let  a:f64 = a.trim().parse().unwrap();
           println!("t = {}sec",(k-d)/(a));

           }else{
              println!("Err");
           }
    }
     let mut a = String::new();
           println!("do you want to continue
press 1 for :yes
press 2 for : no
");
           io::stdin().read_line(&mut a);
           let  a:i8 = a.trim().parse().unwrap();

           if a == 1 {
              println!("First equation of motion");
           }else{
              println!("thanks for using");
              break
           }
  }
 }
 pub fn second_equation_of_motion () {
    loop {
     let mut x = String::new();
       println!("s= vit + 1/2at^2 find");
       println!("plz press 1 for find : s or h");       
       println!("plz press 2 for find : vi");
       println!("plz press 3 for find : a or g");
    io::stdin().read_line(&mut x);
       let  x:i8 = x.trim().parse().unwrap();
       if x == 1 {

           let mut g = String::new();
           println!("the value of vi is in ?
press 1 for :m/sec
press 2 for :km/hour");
           io::stdin().read_line(&mut g);
           let  g:u8 = g.trim().parse().unwrap();


          if g == 1{
              println!("enter the value of vi");
           let mut vi = String::new();
           io::stdin().read_line(&mut vi);
           let  vi:f64 = vi.trim().parse().unwrap();
              
           let mut a = String::new();
           println!("enter the vaue of a or g");
           io::stdin().read_line(&mut a);
           let  a:f64 = a.trim().parse().unwrap();

           let mut t = String::new();
           println!("enter the value of t");
           io::stdin().read_line(&mut t);
           let  t:f64 = t.trim().parse().unwrap();

           let n = t*t*a;
           let n = n/2.00;

           println!("s or h = {} meters",(vi*t)+n);
    }else if g == 2 {
       println!("enter the value of vi in km/h");
           let mut vi = String::new();
           io::stdin().read_line(&mut vi);
           let  vi:f64 = vi.trim().parse().unwrap();

           let d:f64 = vi*1000.00;
           let d = d/3600.00;

           let mut a = String::new();
           println!("enter the vaue of a or g");
           io::stdin().read_line(&mut a);
           let  a:f64 = a.trim().parse().unwrap();

           let mut t = String::new();
           println!("enter the value of t");
           io::stdin().read_line(&mut t);
           let  t:f64 = t.trim().parse().unwrap();

           let n = t*t*a;
           let n = n/2.00;

           println!("s or h = {} meters",(d*t)+(n));
    }else {
       println!("Err");
    }
    }else if x == 2 {

           let mut g = String::new();
           println!("the value of s or h and is in ?
press 1 for :m
press 2 for :km");
           io::stdin().read_line(&mut g);
           let  g:u8 = g.trim().parse().unwrap();

       if g == 1 {
        let mut s = String::new();
           println!("enter the vaue of s or h");
           io::stdin().read_line(&mut s);
           let  s:f64 = s.trim().parse().unwrap();

           let mut t = String::new();
           println!("enter the value of t");
           io::stdin().read_line(&mut t);
           let  t:f64 = t.trim().parse().unwrap();

            
            let mut a = String::new();
           println!("enter the vaue of a or g");
           io::stdin().read_line(&mut a);
           let  a:f64 = a.trim().parse().unwrap();

           let n = t*t*a;
           let n = n/2.0;

            println!("vi = {} m/sec",(n*s)/(t));

       }else if g == 2 {
       let mut s = String::new();
           println!("enter the value of s in km");
           io::stdin().read_line(&mut s);
           let s:f64 = s.trim().parse().unwrap();

           let d = s*1000.0;

           let mut t = String::new();
           println!("enter the value of t");
           io::stdin().read_line(&mut t);
           let  t:f64 = t.trim().parse().unwrap();

            
            let mut a = String::new();
           println!("enter the vaue of a or g");
           io::stdin().read_line(&mut a);
           let  a:f64 = a.trim().parse().unwrap();

           let n = t*t*a;
           let n = n/2.0;

            println!("vi = {} m/sec",(n*d)/(t));
    }else {
       println!("Err");
    }
    }else if x == 3 {
       
           let mut g = String::new();
           println!("the value of and vi is in ?
press 1 for :m/sec
press 2 for :km/h");
           io::stdin().read_line(&mut g);
           let  g:u8 = g.trim().parse().unwrap();

           if g == 1{

            println!("enter the value of vi");
           let mut vi = String::new();
           io::stdin().read_line(&mut vi);
           let  vi:f64 = vi.trim().parse().unwrap();

             let mut s = String::new();
           println!("enter the vaue of s or h");
           io::stdin().read_line(&mut s);
           let  s:f64 = s.trim().parse().unwrap();

           let mut t = String::new();
           println!("enter the value of t");
           io::stdin().read_line(&mut t);
           let  t:f64 = t.trim().parse().unwrap();

           let d = t*t;
           let j = d/2.0;

           println!("a = {}",(s-(vi*t))/j);

           }else if g == 2{
               println!("enter the value of vi in km/h");
           let mut vi = String::new();
           io::stdin().read_line(&mut vi);
           let  vi:f64 = vi.trim().parse().unwrap();

           let l = vi*1000.0;
           let l = l/3600.0;

             let mut s = String::new();
           println!("enter the vaue of 's' or 'h' in m if the value of 's' is in km 
           add three zero in the last ");
           io::stdin().read_line(&mut s);
           let  s:f64 = s.trim().parse().unwrap();

         

           let mut t = String::new();
           println!("enter the value of t");
           io::stdin().read_line(&mut t);
           let  t:f64 = t.trim().parse().unwrap();

           let d = t*t;
           let j = d/2.0;

           println!("a = {}",(s-(l*t))/j);
           }else{
              println!("Err");
        }
 }
  let mut a = String::new();
           println!("do you want to continue
press 1 for :yes
press 2 for : no
");
           io::stdin().read_line(&mut a);
           let  a:i8 = a.trim().parse().unwrap();

           if a == 1 {
              println!("Second equation of motion");
           }else{
              println!("thanks for using");
              break
           }
  }
}
pub fn third_equation_of_motion () {
   loop{
     let mut x = String::new();
       println!("2as=vf^2-vi^2");
       println!("choose following number ");
       println!("plz press 1 for find : a");       
       println!("plz press 2 for find : s");
    io::stdin().read_line(&mut x);
       let  x:u8 = x.trim().parse().unwrap();
    if x == 1{
       let mut g = String::new();
           println!("the value of vf and vi is in ?
press 1 for :m/sec
press 2 for :km/h");
           io::stdin().read_line(&mut g);
           let  g:u8 = g.trim().parse().unwrap();

    if g == 1 {
        let mut s = String::new();
           println!("enter the vaue of s ");
           io::stdin().read_line(&mut s);
           let  s:f64 = s.trim().parse().unwrap();

           let o = s*2.0;

           println!("enter the value of vi");
           let mut vi = String::new();
           io::stdin().read_line(&mut vi);
           let  vi:f64 = vi.trim().parse().unwrap();
           let d = vi*vi;

           println!("enter the value of vf");
           let mut vf = String::new();
           io::stdin().read_line(&mut vf);
           let  vf:f64 = vf.trim().parse().unwrap();
           let k = vf*vf;

           println!("a = {} m/sec^2",(k-d)/o);
    }else if g == 2 {
       
        let mut s = String::new();
           println!("enter the vaue of 's' in 'm' if 's' value is in km 
      add three zero in the last");
           io::stdin().read_line(&mut s);
           let  s:f64 = s.trim().parse().unwrap();

           let o = s*2.0;

           println!("enter the value of vi in km/h");
           let mut vi = String::new();
           io::stdin().read_line(&mut vi);
           let  vi:f64 = vi.trim().parse().unwrap();

           let d:f64 = vi*1000.00;
           let d=d/3600.00;
           let d=d*d;

           println!("enter the value of vf in km/h");
           let mut vf = String::new();
           io::stdin().read_line(&mut vf);
           let  vf:f64 = vf.trim().parse().unwrap();

           let k:f64 = vf*1000.00;
           let k = k/3600.00;
           let k = k*k;

           println!("a = {} m/sec^2",(k-d)/o);
    }else{
       println!("Err");
    }
    }else if x == 2 {
              let mut g = String::new();
           println!("the value of vf and vi is in ?
press 1 for :m/sec
press 2 for :km/h");
           io::stdin().read_line(&mut g);
           let  g:u8 = g.trim().parse().unwrap();

    if g == 1 {
        let mut s = String::new();
           println!("enter the vaue of a ");
           io::stdin().read_line(&mut s);
           let  s:f64 = s.trim().parse().unwrap();

           let o = s*2.0;

           println!("enter the value of vi");
           let mut vi = String::new();
           io::stdin().read_line(&mut vi);
           let  vi:f64 = vi.trim().parse().unwrap();
           let d = vi*vi;

           println!("enter the value of vf");
           let mut vf = String::new();
           io::stdin().read_line(&mut vf);
           let  vf:f64 = vf.trim().parse().unwrap();
           let k = vf*vf;

           println!("s = {} m",(k-d)/o);
    }else if g == 2 {
       
        let mut s = String::new();
           println!("enter the vaue of a ");
           io::stdin().read_line(&mut s);
           let  s:f64 = s.trim().parse().unwrap();

           let o = s*2.0;

           println!("enter the value of vi in km/h");
           let mut vi = String::new();
           io::stdin().read_line(&mut vi);
           let  vi:f64 = vi.trim().parse().unwrap();

           let d:f64 = vi*1000.00;
           let d=d/3600.00;
           let d=d*d;

           println!("enter the value of vf in km/h");
           let mut vf = String::new();
           io::stdin().read_line(&mut vf);
           let  vf:f64 = vf.trim().parse().unwrap();

           let k:f64 = vf*1000.00;
           let k = k/3600.00;
           let k = k*k;

           println!("s = {} m",(k-d)/o);
    }else{
       println!("Err");
    }
    }
     let mut a = String::new();
           println!("do you want to continue
press 1 for :yes
press 2 for : no
");
           io::stdin().read_line(&mut a);
           let  a:i8 = a.trim().parse().unwrap();

           if a == 1 {
              println!("Third equation of motion");
           }else{
              println!("thanks for using");
              break
           }
  }
    }
