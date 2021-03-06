# another-lisp
personal pet project to teach myself Rust PL.<br/>
Also the purpose is to write full lisp intereter.<br/>
I followed this https://stopa.io/post/222 to write the basic interpeter<br/>

#Abilities
1) Basic arithmetic: +,-,*,/<br/>
2) Basic logic     : >, <, =, >=, <=, !, &&, ||<br/>
3) Output          : print<br/>
Prints all passed arguments. Compute it, if required.<br/>
4) If              : if<br/>
If condition is true, then compute first expression,<br/> 
unless try to compute the second one, if it exists.<br/>
5) Const variable definition : def<br/>
Takes 2 arguments: name and value<br/>
