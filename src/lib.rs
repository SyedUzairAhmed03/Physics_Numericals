use std::in;
 fn 1_equation_of_motion () {
        let mut x = String::new();
       println!("vf = vi +at");
       println!("1:vf");       
       println!("2:vi");
       println!("3:a");
       println!("4:t");
    io::stdin().read_line(&mut x);
       let  x:i8 = x.trim().parse().unwrap();
       if x == 1 {

           let mut g = String::new();
           println!("the value of vi is in ?
1:m/sec
2:km/hour");
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
1:m/sec
2:km/hour");
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
1:m/sec
2:km/hour");
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
1:m/sec
2:km/hour");
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
