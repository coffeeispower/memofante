# Memofante
Chegou memofante, uma graaaande ajuda:
- Tem dificuldade em lembrar-se de palavras japonesas que não queria mesmo esquecer?
- Esqueces-te de vocabulário e leituras de kanji que acabaste de aprender?

Memofante é uma CLI para te ajudar a aprender e lembrar palavras japonesas feita em Rust 🦀
![logo do Memofante](./memofante.png)
# Como funciona?

É recomendado usar isto com o
[10ten Japanese Reader](https://github.com/birchill/10ten-ja-reader), 
para que tu possas passar o mouse sobre palavras no navegador e ver a definição.

O Memofante trabalha com o conceito de ✨**palavras descobertas**✨, 
quando tu encontras uma nova palavra na internet que tiveste que usar o 10ten Japanese Reader 
porque não a conhecias, tu podes copiar para o Memofante e revisar mais tarde.

O Memofante tem 4 comandos básicos:
- `memofante add <palavra>`: Adiciona uma nova ✨**palavra descoberta**✨
- `memofante list`: Lista as tuas ✨**palavras descobertas**✨ com sucessos, falhas e taxa de sucesso nas revisões
- `memofante remove <palavra>`: Remove uma ✨**palavra descoberta**✨
- `memofante review`: Inicia uma nova revisão

## Como funcionam as revisões?

O Memofante pergunta o significado ou a leitura de todas as ✨**palavras descobertas**✨, 
a ordem e a frequência com que aparecem depende de quanto tu erras e quantas vezes reviste a palavra, 
se já usaste algo como o Duolingo antes, isso vai parecer familiar.
# Por que o nome "Memofante"?
É uma referência a um medicamento que estava por aí em 2014-2016 em Portugal para melhorar a memória de pessoas com mais de 50 anos, e tinha uma publicidade assim um pouco interessante:

[![Publicidade do Memofante](https://img.youtube.com/vi/mDFfE4VlJz8/0.jpg)](https://www.youtube.com/watch?v=mDFfE4VlJz8)
