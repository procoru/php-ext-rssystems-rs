<?php

// Вывод списка доступных функций нашего расширения
$module = 'rssystems';
$functions = get_extension_funcs($module);
echo "Функции модуля:\n";
foreach ($functions as $func) {
    echo $func . "\n";
}

// Вызов конкретной функции нашего расширения
echo "Результат выполнения:\n" . hello_rssystems() . "\n";
