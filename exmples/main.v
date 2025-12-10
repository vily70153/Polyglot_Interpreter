структура Animal {
  id: ціле,
  name: рядок,
}

функція CallMe(Animal Тварина) {
  змінна t_id = Тварина.id
  змінна t_name = Тварина.name
  друк("ID:", t_id)
  друк("Name:", t_name)
}

CallMe(Animal(1, "Їжачок"))
