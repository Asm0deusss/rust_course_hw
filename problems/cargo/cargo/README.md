В этой задаче вам придётся редактировать сразу несколько файлов и разобраться с тем, как импортировать написанный код из одного файла в другой.


## Состав директории:
* В корневой директории должны находиться файлы:
  * `lib.rs` - объявление доступных извне модулей, имплементации функции `play_games`
  * `trait.rs` - файл с объявлением трейтов, как интерфейсов игр, используемых в проекте. Вам потребуется реализовать 3 трейта: `FairRound`, `UnfairRound` и `InitGame`. 4-ый трейт `Round` имплементирован за вас (он по сути является комбинацией двух трейтов).
  * `config.rs` это файл с описанием общего конфига игр, как набора доступных конфигов заведённых игр.
* В поддиректории `games` должны быть объявлены директории `flip_coin` и `roll_dice`, в каждой из которой должны иметься файлы:
  * `config.rs` - файл с конфигурацией конкретной игры
  * `game.rs` - файл с реализацией трейтов для игры.   

## Суть задачи:
  * Нужно реализовать розыгрыш несколько раундов для поданных на вход конфигураций игр. Есть всего два типа игры:
    * `FlipCoinGame` - подбрасывание монетки
    * `RollDiceGame` - подбрасывание игральной кости

  * Каждая игра содержит в себе (лежат в конфиге) два `u8` числа `players` - идентификаторы игроков. Гарантируется, что идентификаторы игроков в рамках одной игры различны. Функция розыгрыша каждого раунда и всей игры возвращают одно число - идентификатор выигрывшего игрока. Выигрыш в игре определяется тем, кто выиграл последний раунд. Победа в раунде определяется для каждой игры и для каждого режима по отдельности.
  * У каждой игры есть режимы - честный и нечестный, которые предлагается реализовать через трейты `FairRound` и `UnfairRound`. Если раунд проходит в нечестном режиме, то это значит, что один из игроков пытается обмануть соперника и выиграть с помощью запрещённых махинаций.
  * Описание игр:
    * `FlipCoinGame` - игра, очень похожая на обычное подбрасывание монетки. Здесь игроки подкидывают по очереди несколько раз монетки и затем считают в какой доле случаев выпала загаданная сторона монетки. Тот, у кого доля успешных подбрасываний выше - выигрывает.
  
      - `FairRound`:
        В отличие от обычных подбрасываний монеток, в эту игру соревнуются только тогда, когда у одного из игроков празднуется день Рождения. Если доля выпадений монеток одинаковая (вероятности равны по 0.5), то в таком случае выигрывает именниник. После каждого раунда проигравший увеличивает свои навыки подбрасывания и в следующий кон, он изменяет соотношение весов $(x:y)$ на $(x + \delta:y - \delta)$. И в новом кону сравнивается уже это отношение.
      - `UnfairRound`:
        Если в предыдущий кон выиграл именниник, то тогда его оппонент обижается и изменяет перед началом хода монетку, изменяющую его преимущество с весов $(x: y)$ на соотношение $(x + 0.2 : y - 0.2)$. Затем игра проходит в обычном `FairRound` режиме и действуют те же правила.
    * `RollDice` - аналог подбрасывания кубика, вот только в нашем случае он будет несбалансированный, т.е. вероятности выпадения граней могут быть неравными (задаются в конфиге).
      - `FairRound`: 
        Первый игрок играет за нечётные грани, а второй за чётные грани (нумерация с единицы). Выигрывает тот игрок, у кого сумма вероятностей его граней, умноженная на значения граней больше (если значения равны, то выигрывает первый игрок)

      - `UnfairRound`:
        В нечестном раунде последний проигравший игрок находит у противника самую частотную грань, а если таких несколько, то выбирает из них грань с самым большим значением, а у себя находит самую редкую грань и если таких несколько, то выбирает из них с наименьшим значением. После этого, если выбранная вероятность противника больше вероятности проигравшего игрока, то проигравший игрок переклеивает грани кубика (т.е. меняет выбранные вероятности местами). После этого игра продолжается в обычном формате.

  - Что нужно сделать?
    1. Вам нужно реализовать функцию `play_game`, которая принимает в себя интерфейс `Round` игры и количество честных и нечестных раундов, запустив в этой функции заданное количество раз честных раундов и после этого - заданное количество раз нечестных раундов этой игры. Функция возвращает идентификатор выигрывшего игрока (он всегда будет, если хотя бы один раунд был сыгран).

    2. Вам также надо реализовать функцию `play_games`, в которой необходимо будет по заданным конфигурациям в формате `json` в строках иниациализировать игры и вернуть результаты всех игр в векторе.

    3. Чтобы реализовать `play_games`, вам нужно научиться превращать строковое представление конфига в `enum` из структур. Для этого в `config.rs` в корневой директории добавлена строчка `#[serde]...`, она описывает по каким правилам будет происходить парсинг. В `Cargo.toml` файле вам добавлены две библиотеки:
        ```
        serde = { version = "*", features = ["derive"] }
        serde_json = "*"
        ```
        С помощью конструкции `#[derive(Deserialize)]` и функции `serde_json::from_str::<Type>(str_data).unwrap()` вы можете превращать `json` конфиг в структуру `Type` с такими же полями у которой реализован трейт `Deserialize`.