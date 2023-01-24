/*
 * Copyright (C) 2015 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package com.example.android.bluetoothadvertisements;

import static android.bluetooth.BluetoothGatt.GATT_SUCCESS;
import static android.bluetooth.BluetoothGattCharacteristic.PROPERTY_INDICATE;
import static android.bluetooth.BluetoothGattCharacteristic.PROPERTY_NOTIFY;
import static android.bluetooth.BluetoothGattCharacteristic.PROPERTY_READ;
import static android.bluetooth.BluetoothGattCharacteristic.PROPERTY_WRITE;

import android.Manifest;
import android.app.AlarmManager;
import android.app.AlertDialog;
import android.app.Notification;
import android.app.NotificationChannel;
import android.app.NotificationManager;
import android.app.PendingIntent;
import android.bluetooth.BluetoothAdapter;
import android.bluetooth.BluetoothDevice;
import android.bluetooth.BluetoothGatt;
import android.bluetooth.BluetoothGattCallback;
import android.bluetooth.BluetoothGattCharacteristic;
import android.bluetooth.BluetoothGattDescriptor;
import android.bluetooth.BluetoothManager;
import android.bluetooth.BluetoothProfile;
import android.content.BroadcastReceiver;
import android.content.Context;
import android.content.DialogInterface;
import android.content.Intent;
import android.content.pm.PackageManager;
import android.os.Build;
import android.os.Bundle;
import android.os.Handler;
import android.os.Looper;
import android.support.annotation.RequiresApi;
import android.support.v4.app.ActivityCompat;
import android.support.v4.app.FragmentActivity;
import android.support.v4.app.FragmentTransaction;
import android.support.v4.app.NotificationCompat;
import android.support.v4.app.NotificationManagerCompat;
import android.support.v4.content.ContextCompat;
import android.support.v7.app.AppCompatActivity;
import android.util.Log;
import android.view.View;
import android.view.WindowManager;
import android.widget.EditText;
import android.widget.TextView;
import android.widget.Toast;

import java.util.List;
import java.util.Queue;
import java.util.UUID;
import java.util.concurrent.ConcurrentLinkedQueue;
import java.util.concurrent.TimeUnit;

/**
 * Setup display fragments and ensure the device supports Bluetooth.
 */
interface Something {
    void callbackCall(BluetoothDevice device);
}
interface Warning {
    void warningCallback();
}

public class MainActivity extends FragmentActivity {
    private Handler bleHandler;
    private BluetoothAdapter mBluetoothAdapter;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        bleHandler = new Handler(Looper.getMainLooper());
        setContentView(R.layout.activity_main);
        setTitle(R.string.activity_main_title);
        if ((ContextCompat.checkSelfPermission(this, Manifest.permission.ACCESS_FINE_LOCATION)
                != PackageManager.PERMISSION_GRANTED)) {
            // Permission is not granted
            ActivityCompat.requestPermissions(MainActivity.this, new String[]{Manifest.permission.ACCESS_FINE_LOCATION}, 69);
        }
        ActivityCompat.requestPermissions(MainActivity.this, new String[]{Manifest.permission.ACCESS_FINE_LOCATION}, 69);

        createNotificationChannel();




        if (savedInstanceState == null) {

            mBluetoothAdapter = ((BluetoothManager) getSystemService(Context.BLUETOOTH_SERVICE))
                    .getAdapter();

            // Is Bluetooth supported on this device?
            if (mBluetoothAdapter != null) {

                // Is Bluetooth turned on?
                if (mBluetoothAdapter.isEnabled()) {

                    // Are Bluetooth Advertisements supported on this device?
                    if (mBluetoothAdapter.isMultipleAdvertisementSupported()) {

                        // Everything is supported and enabled, load the fragments.
                        setupFragments();

                    } else {

                        // Bluetooth Advertisements are not supported.
                        showErrorText(R.string.bt_ads_not_supported);
                    }
                } else {

                    // Prompt user to turn on Bluetooth (logic continues in onActivityResult()).
                    Intent enableBtIntent = new Intent(BluetoothAdapter.ACTION_REQUEST_ENABLE);
                    startActivityForResult(enableBtIntent, Constants.REQUEST_ENABLE_BT);
                }
            } else {

                // Bluetooth is not supported.
                showErrorText(R.string.bt_not_supported);
            }

        }
    }
    private void createNotificationChannel() {
        // Create the NotificationChannel, but only on API 26+ because
        // the NotificationChannel class is new and not in the support library
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            //CharSequence name = getString(R.string);
            //String description = getString(R.string.channel_description);
            int importance = NotificationManager.IMPORTANCE_DEFAULT;
            NotificationChannel channel = new NotificationChannel("warnings", "worning", importance);
            channel.setDescription("warning");
            // Register the channel with the system; you can't change the importance
            // or other notification behaviors after this
            NotificationManager notificationManager = getSystemService(NotificationManager.class);
            notificationManager.createNotificationChannel(channel);
        }
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        super.onActivityResult(requestCode, resultCode, data);
        switch (requestCode) {
            case Constants.REQUEST_ENABLE_BT:

                if (resultCode == RESULT_OK) {

                    // Bluetooth is now Enabled, are Bluetooth Advertisements supported on
                    // this device?
                    if (mBluetoothAdapter.isMultipleAdvertisementSupported()) {

                        // Everything is supported and enabled, load the fragments.
                        setupFragments();

                    } else {

                        // Bluetooth Advertisements are not supported.
                        showErrorText(R.string.bt_ads_not_supported);
                    }
                } else {

                    // User declined to enable Bluetooth, exit the app.
                    Toast.makeText(this, R.string.bt_not_enabled_leaving,
                            Toast.LENGTH_SHORT).show();
                    finish();
                }

            default:
                super.onActivityResult(requestCode, resultCode, data);
        }
    }

    private void setupFragments() {
        FragmentTransaction transaction = getSupportFragmentManager().beginTransaction();

        ScannerFragment scannerFragment = new ScannerFragment();
        // Fragments can't access system services directly, so pass it the BluetoothAdapter
        scannerFragment.setBluetoothAdapter(mBluetoothAdapter);
        transaction.replace(R.id.scanner_fragment_container, scannerFragment);
        scannerFragment.something = new Something() {
            @Override
            public void callbackCall(BluetoothDevice device) {
                makeDialog(device);
            }
        };

        AdvertiserFragment advertiserFragment = new AdvertiserFragment();
        transaction.replace(R.id.advertiser_fragment_container, advertiserFragment);

        transaction.commit();
    }

    private void showErrorText(int messageId) {
        TextView view = (TextView) findViewById(R.id.error_textview);
        view.setText(getString(messageId));
    }

    BluetoothGatt gatt;
    @RequiresApi(api = Build.VERSION_CODES.O)
    private void warnUser(){
       NotificationCompat.Builder builder = new NotificationCompat.Builder(this, "warnings")
                .setSmallIcon(R.drawable.ic_stat_name)
                .setContentTitle("Warning")
                .setContentText("Potential seizure detected")
                .setPriority(NotificationCompat.PRIORITY_DEFAULT);
        NotificationManagerCompat notificationManager = NotificationManagerCompat.from(this);

// notificationId is a unique int for each notification that you must define
        notificationManager.notify(420, builder.build());
    }
    private void showSuccess(){
        bleHandler.post(new Runnable() {
                            @Override
                            public void run() {
                                Toast.makeText(getApplicationContext(), "Connection successful", Toast.LENGTH_LONG).show();
                            }
                        });
    }
    private void showFail(){
        bleHandler.post(new Runnable() {
            @Override
            public void run() {
                Toast.makeText(getApplicationContext(), "Connection not successful, try again", Toast.LENGTH_LONG).show();
            }
        });
    }


    public AlertDialog.Builder makeDialog(BluetoothDevice device) {
        android.app.AlertDialog.Builder alert = new android.app
                .AlertDialog.Builder(this, R.style.AlertDialog);
        final EditText edittext = new EditText(this);
        final BluetoothDevice finaldevice = device;
        alert.setMessage("Enter code:");
        alert.setTitle("Authentication needed");
        edittext.setTextColor(0xFF000000);
        alert.setView(edittext);

        alert.setPositiveButton("OK", new DialogInterface.OnClickListener() {
            @RequiresApi(api = Build.VERSION_CODES.M)
            public void onClick(DialogInterface dialog, int whichButton) {
                String userInput = edittext.getText().toString();
                Log.i("BLE", userInput);
                gatt = openComms(finaldevice);
                cmdQueue = new ConcurrentLinkedQueue<>();
                writeCharacteristic();
            }
        });

        alert.setNegativeButton("Cancel", new DialogInterface.OnClickListener() {
            public void onClick(DialogInterface dialog, int whichButton) {
            }
        });

        alert.show();
        return alert;

    }

    private Boolean warned = false;

    public void spawnUserNotification() {
        if (warned == false) {
            Log.i("BLE", "bor verkligen spawn dialog");
            android.app.AlertDialog.Builder alert = new android.app
                    .AlertDialog.Builder(this, R.style.AlertDialog);
            alert.setMessage("WARNING");
            alert.setTitle("WARNING");

            alert.setPositiveButton("Yes", new DialogInterface.OnClickListener() {
                @RequiresApi(api = Build.VERSION_CODES.M)
                public void onClick(DialogInterface dialog, int whichButton) {
                    warned = false;
                }
            });

            alert.setNegativeButton("No", new DialogInterface.OnClickListener() {
                public void onClick(DialogInterface dialog, int whichButton) {
                    warned = false;
                }
            });

            alert.show();
            warned = true;
        }
    }

    @RequiresApi(api = Build.VERSION_CODES.M)
    private BluetoothGatt openComms(BluetoothDevice device) {
        BluetoothGattCallback bluetoothGattCallback = new BluetoothGattCallback() {
            @Override
            public void onCharacteristicWrite(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, int status) {
                Log.i("BLE", "Characteristic written");
                completedCommand();
            }

            Warning warning;

            public Warning getWarning() {
                return warning;
            }
            @Override
            public void onDescriptorWrite(BluetoothGatt gatt, final BluetoothGattDescriptor descriptor, final int status){
                Log.i("BLE", "Descriptor written");
                completedCommand();
            }
            @Override
            public void onConnectionStateChange(BluetoothGatt gatt, int status, int newState) {
                if (status == GATT_SUCCESS) {
                    if (newState == BluetoothProfile.STATE_CONNECTED) {
                        gatt.discoverServices();
                        Log.i("ble", "Services discovered");
                        showSuccess();
                    } else {
                        gatt.close();
                    }
                } else {
                    Log.i("BLE", Integer.toString(status));
                    showFail();
                    gatt.close();
                }
            }


            @RequiresApi(api = Build.VERSION_CODES.O)
            @Override
            public void onCharacteristicChanged(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic) {
                Log.i("BLE", "Characteristic change detected");
                completedCommand();
                Log.i("BLE", String.format("%s", characteristic.getUuid()));
                byte[] arr = characteristic.getValue();
                String res = "";
                for(int i = 0; i<arr.length; i++){
                    res+= String.format("%c", arr[i]);
                }
                Log.i("BLE", res);
                warnUser();
            }

            @Override
            public void onCharacteristicRead(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, int status) {
                Log.i("BLE", "Characteristic Read :DDD");
                completedCommand();
            }

            @Override
            public void onServicesDiscovered(BluetoothGatt gatt, int status) {
                List<BluetoothGattCharacteristic> characteristicsone;
                characteristicsone = gatt.getServices().get(0).getCharacteristics();
                Log.i("BLE", "services discovered");
                Log.i("BLE", Integer.toString(gatt.getServices().size()));
                Log.i("BLE", "characteristics1");
                Log.i("BLE", Integer.toString(characteristicsone.size()));
                for (int i = 0; i < characteristicsone.size(); i++) {
                    Log.i("BLE", Integer.toString(i));
                    if ((characteristicsone.get(i).getProperties() & PROPERTY_WRITE) == 0) {
                        Log.i("BLE", "Characteristic cannot be read");
                    } else {
                        Log.i("BLE", "Characteristic can be read");
                    }
                    Log.i("BLE", characteristicsone.get(i).getUuid().toString());
                }
                List<BluetoothGattCharacteristic> characteristicstwo;
                characteristicstwo = gatt.getServices().get(1).getCharacteristics();
                Log.i("BLE", "characteristics2");
                Log.i("BLE", Integer.toString(characteristicstwo.size()));
                for (int i = 0; i < characteristicstwo.size(); i++) {
                    Log.i("BLE", Integer.toString(i));
                    if ((characteristicstwo.get(i).getProperties() & PROPERTY_WRITE) == 0) {
                        Log.i("BLE", "Characteristic cannot be read");
                    } else {
                        Log.i("BLE", "Characteristic can be read");
                    }
                    Log.i("BLE", characteristicstwo.get(i).getUuid().toString());
                }
                List<BluetoothGattCharacteristic> characteristicsthree;
                characteristicsthree = gatt.getServices().get(2).getCharacteristics();
                Log.i("BLE", "characteristics3");
                Log.i("BLE", Integer.toString(characteristicsthree.size()));
                for (int i = 0; i < characteristicsthree.size(); i++) {
                    Log.i("BLE", Integer.toString(i));
                    if ((characteristicsthree.get(i).getProperties() & PROPERTY_WRITE) == 0) {
                        Log.i("BLE", Integer.toString(characteristicsthree.get(i).getProperties()));
                        Log.i("BLE", "Characteristic cannot be read");
                    } else {
                        Log.i("BLE", Integer.toString(characteristicsthree.get(i).getProperties()));
                        Log.i("BLE", "Characteristic can be read");
                    }
                    Log.i("BLE", characteristicsthree.get(i).getUuid().toString());
                }
                setNotify(characteristicsthree.get(0),true);
                initQueue();
            }
        };
        BluetoothGatt gatt = device.connectGatt(this, false, bluetoothGattCallback, BluetoothDevice.TRANSPORT_LE);
        return gatt;
    }

    private Queue<Runnable> cmdQueue;
    private boolean mutex = true;

    public boolean readCharacteristic() {
        Log.i("BLE","Reading Characterisitc");
        boolean result = cmdQueue.add(new Runnable() {
            @Override
            public void run() {
                final BluetoothGattCharacteristic interesting = gatt.getServices().get(2).getCharacteristics().get(0);
                Log.i("BLE","Reading Characterisitc runnable pre");
                gatt.readCharacteristic(interesting);
                Log.i("BLE","Reading Characterisitc runnable post");
                //setNotify(interesting, true);
            }
        });
        if (result) {
            nextCommand();

        }
        return result;
    }

    public boolean writeCharacteristic() {
        boolean result = cmdQueue.add(new Runnable() {
            @Override
            public void run() {
                final BluetoothGattCharacteristic interesting = gatt.getServices().get(2).getCharacteristics().get(1);
                interesting.setValue("hello");
                gatt.writeCharacteristic(interesting);
            }
        });
        if (result) {
            nextCommand();

        }
        return result;
    }

    private void nextCommand() {
        if (mutex) {
            return;
        } else {
            if (cmdQueue.size() > 0) {
                final Runnable blerun = cmdQueue.peek();
                mutex = true;
                bleHandler.post(new Runnable() {
                    @Override
                    public void run() {
                        blerun.run();
                    }
                });
            }
        }
    }

    private void initQueue() {
        mutex = false;
        nextCommand();
    }

    private void completedCommand() {
        mutex = false;
        cmdQueue.poll();
        nextCommand();
    }

    private final String CCC_DESCRIPTOR_UID = "00002902-0000-1000-8000-00805f9b34fb";

    public boolean setNotify(BluetoothGattCharacteristic characteristic,final boolean enable) {
        //BluetoothGattCharacteristic characteristic = gatt.getServices().get(2).getCharacteristics().get(0);
        Log.i("BLE", "INSIDE SET NOTI");
        if (characteristic == null) {
            Log.i("BLE", "Char null, no notify");
            return false;
        }

        final BluetoothGattDescriptor descriptor = characteristic.getDescriptor(UUID.fromString(CCC_DESCRIPTOR_UID));
        if (descriptor == null) {
            return false;
        }
        byte[] value;
        int properties = characteristic.getProperties();
        if ((properties & PROPERTY_NOTIFY) > 0) {
            Log.i("BLE", "ENABLE NOTI VALUE");
            value = BluetoothGattDescriptor.ENABLE_NOTIFICATION_VALUE;
        } else if ((properties & PROPERTY_INDICATE) > 0) {
            Log.i("BLE", "ENABLE INDI VALUE");
            value = BluetoothGattDescriptor.ENABLE_INDICATION_VALUE;
        } else {
            Log.i("BLE", "no notify or indicate property");
            return false;
        }
        final byte[] finalValue = enable ? value : BluetoothGattDescriptor.DISABLE_NOTIFICATION_VALUE;

        boolean result = cmdQueue.add(new Runnable() {
            @Override
            public void run() {
                if (!gatt.setCharacteristicNotification(descriptor.getCharacteristic(), enable)) {
                }
                descriptor.setValue(finalValue);
                boolean result;
                result = gatt.writeDescriptor(descriptor);
                if (!result) {
                    completedCommand();
                }
            }
        });
        if (result) {
            nextCommand();
        }
        return result;
    }
}