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
5) Variable (re)definition : def<br/>
Takes 2 arguments: name and value<br/>
6) Lambdas: (fn (a) (+ a 1))<br/>
Also you can execute it:<br/>

```lisp
(print
    (fn (a) (+ a 1) 2)
)
```
You can define it to use somewhere else:<br/>
```lisp
(defn lmd
  (fn (arg) (print arg))
)

(lmd! (2))
```

7)One line comment:<br>
```lisp 
#(print "it won't never be printed")
```
