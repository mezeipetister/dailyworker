import 'dart:async';
import 'dart:core';
import 'package:confirm_dialog/confirm_dialog.dart';
import 'package:anykexport/table.dart';
import 'package:file_selector/file_selector.dart';
import 'package:flutter/material.dart';
import 'package:anykexport/src/rust/api/simple.dart';
import 'package:anykexport/src/rust/api/worker.dart';
import 'package:anykexport/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
        home: Scaffold(
      body: Center(
        child: Demo(),
      ),
    ));
  }
}

class Demo extends StatefulWidget {
  @override
  _State createState() => _State();
}

class _State extends State<Demo> {
  int count = 0;
  List<Worker> workers = List.empty();
  String query = "";
  late FocusNode myFocusNode;

  @override
  void initState() {
    // TODO: implement initState
    super.initState();
    workers = getWorkers();
    myFocusNode = FocusNode();
  }

  @override
  void dispose() {
    // Clean up the focus node when the Form is disposed.
    myFocusNode.dispose();

    super.dispose();
  }

  Future<void> addEmptyWorker() async {
    debugPrint('Adding empty worker');
    if (!await confirm(context,
        title: Text('Biztos?'),
        content: Text('Biztosan létrehozod az új munkavállalót?'),
        textOK: Text('Létrehozás'),
        textCancel: Text('Mégsem'))) {
      return;
    }
    Worker worker = getEmptyWorker();
    setState(() {
      addWorker(worker: worker);
      workers = getWorkers();
    });
  }

  Future<void> deleteWorker(Worker worker) async {
    if (!await confirm(context,
        title: Text('Biztosan törlöd?'),
        content: Text('Nincs mód visszavonni a műveletet!'),
        textOK: Text('Törlés'),
        textCancel: Text('Mégsem'))) {
      return;
    }
    setState(() {
      removeWorkerApi(worker: worker);
      workers = getWorkers();
      Navigator.of(context).pop();
    });
  }

  void setQuery(String q) {
    setState(() {
      query = q;
    });
  }

  void localUpdateWorker(Worker worker) {
    print("Hello");
    updateWorker(worker: worker);
    setState(() {
      workers = getWorkers();
    });
  }

  void incrementCounter() {
    setState(() {
      //increment();
      //count = getCounter();
    });
  }

  Future<void> exportXml() async {
    final String? directoryPath = await getDirectoryPath();
    if (directoryPath == null) {
      ScaffoldMessenger.of(context).showSnackBar(const SnackBar(
        backgroundColor: Colors.red,
        content: Text("Sikertelen export."),
      ));
    } else {
      exportXmlApi(to: directoryPath);
      ScaffoldMessenger.of(context).showSnackBar(const SnackBar(
        backgroundColor: Colors.green,
        content: Text("Sikeres mentés!"),
      ));
    }
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      child: Row(
        children: [
          Expanded(
              child: Padding(
            padding: EdgeInsets.all(16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                Padding(
                  padding: EdgeInsets.all(16),
                  child: Row(
                    mainAxisSize: MainAxisSize.max,
                    mainAxisAlignment: MainAxisAlignment.spaceAround,
                    children: [
                      OutlinedButton(
                          onPressed: addEmptyWorker,
                          child: Text('Új'),
                          style: ElevatedButton.styleFrom(
                            backgroundColor: Colors.white,
                            foregroundColor: Colors.black,
                            shadowColor: Colors.yellow,
                          )),
                      SizedBox(
                        width: 300,
                        height: 40,
                        child: TextField(
                            onSubmitted: (q) {
                              setQuery(q);
                              myFocusNode.requestFocus();
                            },
                            focusNode: myFocusNode,
                            obscureText: false,
                            decoration: InputDecoration(
                              border: OutlineInputBorder(),
                              labelText: 'Szűrés',
                            ),
                            style: TextStyle(fontSize: 12, height: 1)),
                      )
                    ],
                  ),
                ),
                Expanded(
                    child: DataTable2SimpleDemo(
                  query: query,
                  full: true,
                  workers: workers,
                  on_change: (Worker w) => {localUpdateWorker(w)},
                  delete: (Worker w) => {deleteWorker(w)},
                )),
              ],
            ),
          )),
          Expanded(
              child: Padding(
            padding: EdgeInsets.all(15),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.start,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: <Widget>[
                Padding(
                  padding: EdgeInsets.all(16),
                  child: Row(
                    mainAxisSize: MainAxisSize.max,
                    mainAxisAlignment: MainAxisAlignment.start,
                    children: [
                      ElevatedButton(
                        onPressed: () => exportXml(),
                        style: ElevatedButton.styleFrom(
                          backgroundColor: Colors.lightGreen,
                          foregroundColor: Colors.white,
                        ),
                        child: const Text(
                          'Export',
                          style: TextStyle(fontWeight: FontWeight.w400),
                        ),
                      ),
                    ],
                  ),
                ),
                Text(
                  'Kiválasztott munkavállalók',
                  style: Theme.of(context).textTheme.headlineLarge,
                ),
                Expanded(
                  child: DataTable2SimpleDemo(
                    query: query,
                    full: false,
                    workers: workers,
                    on_change: (Worker w) => localUpdateWorker(w),
                    delete: (Worker w) => deleteWorker(w),
                  ),
                )
              ],
            ),
          )),
        ],
      ),
    );
  }
}
