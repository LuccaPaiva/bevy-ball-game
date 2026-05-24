Projeto copiando https://www.youtube.com/watch?v=izhFutJiZgo&list=PLVnntJRoP85JHGX7rGDu6LaF3fmDDbqyd&index=2

Usando assets de:
- https://www.kenney.nl/assets/sci-fi-sounds
- https://www.kenney.nl/assets/impact-sounds
- https://www.kenney.nl/assets/interface-sounds
- https://www.kenney.nl/assets/rolling-ball-assets

No original, o autor separa confine_enemy_movement, update_enemy_direction e enemy_movement em funcoes separadas.
Mas eu achei melhor juntar tudo na move_direct_confine_enemies pq acho que dava uns bugs de concorrencia, do objeto andar e mudar de direção, ai ele ficava "flicando" nas paredes. Parece ter resolvido.



Referencias

Estados: https://taintedcoders.com/bevy/how-to/use-app-state
