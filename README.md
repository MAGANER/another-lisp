# another-lisp
personal pet project to teach myself Rust PL.<br/>
Also the purpose is to write full lisp intereter.<br/>
I followed this https://stopa.io/post/222 to write the several parts of interpeter<br/>

Future goal is to create language to use<br/>
instead of bash and its analogues.<br/>

# Abilities
1) Basic arithmetic: ```+```, ```-```, ```*``` ,```/``` <br/>
2) Basic logic     : ```>```, ```<```, ```=```, ```>=```, ```<=```, ```!```, ```&&```, ```||``` <br/>
3) Output          : ```print```<br/>
Prints all passed arguments. Compute it, if required.<br/>
4) If              : ```if```<br/>
If condition is true, then compute first expression,<br/> 
unless try to compute the second one, if it exists.<br/>
5) Variable (re)definition : ```def```<br/>
Takes 2 arguments: name and value<br/>
6) Lambdas: ```(fn (a) (+ a 1))```<br/>
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
8) List operations:<br/>
### head
returns first element of list.<br>
```lisp
(print
    (head (1 2 4) )
)
```
result: 1<br>

### tail
returns all list elements, except the first one.<br/>
```lisp
(print
    (tail (1 2 4) )
)
```
result:(2 4)

### concat
unites different value into the list.
```lisp
(print
    (concat (1 2 'hey' True (False 645)) )
)
```
result:(1 2 'hey' True False 645)<br/>

9) Type checking<br/>
To find out the type of value you can use ```type``` operation.<br/>
### Example 1:
```lisp
(print (type 1))
```
result:'Number'

### Example 2:
```lisp
(print (type (1 4 True)))
```
result:'List'

### Example 3:
```lisp
(print (type 1 'hey' False))
```
result:('Number'  'String' 'Bool)
