import 'dart:core';
import 'package:data_table_2/data_table_2.dart';
import 'package:flutter/material.dart';
import 'package:anykexport/src/rust/api/worker.dart';

/// Example without a datasource
class DataTable2SimpleDemo extends StatelessWidget {
  List<Worker> workers;
  Function on_change;
  Function delete;
  bool full;
  String query;

  DataTable2SimpleDemo(
      {super.key,
      required this.workers,
      required this.on_change,
      required this.delete,
      required this.full,
      required this.query});

  @override
  Widget build(BuildContext context) {
    return Padding(
        padding: const EdgeInsets.all(16),
        child: DataTable2(
            columnSpacing: 12,
            horizontalMargin: 12,
            minWidth: 300,
            columns: full
                ? [
                    const DataColumn2(
                      label: Text(''),
                      size: ColumnSize.S,
                    ),
                    const DataColumn2(label: Text('Név'), size: ColumnSize.L),
                    const DataColumn2(
                      label: Text('Lakcím'),
                    ),
                    const DataColumn2(
                      label: Text('TAJ'),
                    ),
                    const DataColumn2(
                      label: Text('Adószám'),
                    ),
                    const DataColumn2(
                      label: Text('Anyja neve'),
                    ),
                    const DataColumn2(label: Text(''))
                  ]
                : [
                    const DataColumn2(
                      label: Text(''),
                      size: ColumnSize.S,
                    ),
                    const DataColumn2(
                      label: Text('Name'),
                    ),
                    const DataColumn2(label: Text(''))
                  ],
            rows: workers
                .where((element) => full
                    ? element.id.toString().contains(query) ||
                        element.name.toLowerCase().contains(query)
                    : element.isSelected)
                .map((w) => DataRow(
                    cells: full
                        ? [
                            DataCell(Checkbox(
                              onChanged: (b) {
                                var nw = w;
                                nw.isSelected = b!;
                                on_change(nw);
                              },
                              value: w.isSelected,
                            )),
                            DataCell(Text(w.name)),
                            DataCell(Text('${w.zip} ${w.city} ${w.street}')),
                            DataCell(Text(w.taxnumber)),
                            DataCell(Text(w.taj)),
                            DataCell(Text(w.mothersname)),
                            DataCell(EditButton(
                                worker: w,
                                delete: (d) {
                                  delete(d);
                                },
                                on_save: (d) {
                                  on_change(d);
                                }))
                          ]
                        : [
                            DataCell(Checkbox(
                              onChanged: (b) {
                                var nw = w;
                                nw.isSelected = b!;
                                on_change(nw);
                              },
                              value: w.isSelected,
                            )),
                            DataCell(Text(w.name)),
                            DataCell(EditButton(
                                worker: w,
                                delete: (d) => delete(d)
                                ,
                                on_save: (d) => on_change(d)
                            ))
                          ]))
                .toList()));
  }
}

class EditButton extends StatelessWidget {
  Worker worker;
  Function(Worker) on_save;
  Function(Worker) delete;

  EditButton(
      {super.key,
      required this.worker,
      required this.on_save,
      required this.delete});

  @override
  Widget build(BuildContext context) {
    return IconButton(
      onPressed: () => _dialogBuilder(context),
      icon: const Icon(Icons.edit),
    );
  }

  Future<void> _dialogBuilder(BuildContext context) {
    return showDialog<void>(
      context: context,
      builder: (BuildContext context) {
        return EditDialog(
          worker: worker,
          on_save: on_save,
          delete: delete,
        );
      },
    );
  }
}

class EditDialog extends StatelessWidget {
  Worker worker;
  Function(Worker) on_save;
  Function(Worker) delete;
  final _formKey = GlobalKey<FormState>();

  EditDialog(
      {super.key, required this.worker, required this.on_save, required this.delete}) {
    on_save = on_save;
    delete = delete;
    worker = worker.cloned();
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Szerkesztés'),
      scrollable: true,
      content: Form(
        key: _formKey,
        child: Column(
          children: <Widget>[
            TextFormField(
              decoration: const InputDecoration(
                labelText: 'Név',
              ),
              initialValue: worker.name,
              validator: (value) {
                if (value!.isEmpty) {
                  return 'A név kötelező';
                }
                return null;
              },
              onSaved: (v) {
                worker.name = v!;
                print('Saved!');
              },
            ),
            TextFormField(
              decoration: const InputDecoration(
                labelText: 'Születésnap',
              ),
              initialValue: worker.birthdate,
              validator: (value) {
                if (value!.isEmpty) {
                  return 'Kötelező a születésnap';
                }
                final regex = RegExp(
                    r'^\d{4}\-(0[1-9]|1[012])\-(0[1-9]|[12][0-9]|3[01])$');
                if (!regex.hasMatch(value)) {
                  return 'Kötelező formátum: ÉÉÉÉ-HH-NN';
                }
                return null;
              },
              onSaved: (v) {
                worker.birthdate = v!;
              },
            ),
            TextFormField(
              decoration: const InputDecoration(
                labelText: 'Születés helye',
              ),
              initialValue: worker.birthplace,
              validator: (value) {
                if (value!.isEmpty) {
                  return 'Kötelező kitölteni';
                }
                return null;
              },
              onSaved: (v) {
                worker.birthplace = v!;
              },
            ),
            TextFormField(
              decoration: const InputDecoration(
                labelText: 'Anyja neve',
              ),
              initialValue: worker.mothersname,
              validator: (value) {
                if (value!.isEmpty) {
                  return 'Kötelező kitölteni';
                }
                return null;
              },
              onSaved: (v) {
                worker.mothersname = v!;
              },
            ),
            TextFormField(
              decoration: const InputDecoration(
                labelText: 'Irányítószám',
              ),
              initialValue: worker.zip,
              validator: (value) {
                if (value!.isEmpty) {
                  return 'Kötelező az irányítószám';
                }
                if (double.tryParse(value) == null) {
                  return 'Csak szám lehet!';
                }
                if (value.length != 4) {
                  return '4 számjegy kötelező';
                }
                return null;
              },
              onSaved: (v) {
                worker.zip = v!;
              },
            ),
            TextFormField(
              decoration: const InputDecoration(
                labelText: 'Település',
              ),
              initialValue: worker.city,
              validator: (value) {
                if (value!.isEmpty) {
                  return 'Kötelező kitölteni';
                }
                return null;
              },
              onSaved: (v) {
                worker.city = v!;
              },
            ),
            TextFormField(
              decoration: const InputDecoration(
                labelText: 'Utca, házszám',
              ),
              initialValue: worker.street,
              validator: (value) {
                if (value!.isEmpty) {
                  return 'Kötelező kitölteni';
                }
                return null;
              },
              onSaved: (v) {
                worker.street = v!;
              },
            ),
            TextFormField(
              decoration: const InputDecoration(
                labelText: 'TAJ',
              ),
              initialValue: worker.taj,
              validator: (value) {
                if (value!.isEmpty) {
                  return 'Kötelező kitölteni';
                }
                final regex = RegExp(r'\b[0-9]{3} ?[0-9]{3} ?[0-9]{3}\b');
                if (!regex.hasMatch(value)) {
                  return 'Nem megfelelő TAJ formátum!';
                }
                return null;
              },
              onSaved: (v) {
                worker.taj = v!;
              },
            ),
            TextFormField(
              decoration: const InputDecoration(
                labelText: 'Adószám',
              ),
              initialValue: worker.taxnumber,
              validator: (value) {
                if (value!.isEmpty) {
                  return 'Kötelező kitölteni';
                }
                final regex = RegExp(
                    r'^(\d{7})(\d)\-([1-5])\-(0[2-9]|[13][0-9]|2[02-9]|4[0-4]|51)$');
                if (!regex.hasMatch(value)) {
                  return 'Formátum hiba: XXXXXXXX-X-XX';
                }
                return null;
              },
              onSaved: (v) {
                worker.taxnumber = v!;
              },
            ),
          ],
        ),
      ),
      actions: <Widget>[
        TextButton(
          style: TextButton.styleFrom(
            textStyle: Theme.of(context).textTheme.labelLarge,
          ),
          child: const Text('Mégse'),
          onPressed: () {
            Navigator.of(context).pop();
          },
        ),
        TextButton(
          style: TextButton.styleFrom(
            textStyle: Theme.of(context).textTheme.labelLarge,
          ),
          child: const Text('Mentés'),
          onPressed: () {
            if (_formKey.currentState!.validate()) {
              _formKey.currentState?.save();
              print('Name is: ${worker.name}');
              on_save(worker);
              Navigator.of(context).pop();
            }
          },
        ),
        TextButton(
          style: TextButton.styleFrom(
            textStyle: Theme.of(context).textTheme.labelLarge,
          ),
          child: const Text(
            'Törlés',
            style: TextStyle(color: Colors.red),
          ),
          onPressed: () {
            delete(worker);
          },
        ),
      ],
    );
  }
}
