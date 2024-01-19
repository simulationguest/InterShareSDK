package com.julian_baumann.data_rct.bluetoothLowEnergy

import android.Manifest
import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothServerSocket
import android.bluetooth.BluetoothSocket
import android.content.Context
import android.content.pm.PackageManager
import android.util.Log
import androidx.core.app.ActivityCompat
import java.io.IOException

class L2CapServerThread(context: Context, adapter: BluetoothAdapter) : Thread() {
    private val serverSocket: BluetoothServerSocket

    init {
        if (ActivityCompat.checkSelfPermission(context, Manifest.permission.BLUETOOTH_ADVERTISE) != PackageManager.PERMISSION_GRANTED) {
            throw BlePermissionNotGrantedException()
        }

        serverSocket = adapter.listenUsingInsecureL2capChannel()
    }

    public fun getPsm(): Int {
        return serverSocket.psm
    }

    override fun run() {
        var socket: BluetoothSocket?

        while (true) {
            try {
                socket = serverSocket.accept()
            } catch (e: IOException) {
                Log.e("L2CAP", "Socket's accept() method failed", e)
                break
            }

            if (socket != null) {
                manageConnectedSocket(socket)
            }
        }
    }

    private fun manageConnectedSocket(socket: BluetoothSocket) {
        val inputStream = socket.inputStream
        val outputStream = socket.outputStream
    }

    fun close() {
        try {
            serverSocket.close()
        } catch (e: IOException) {
            Log.e("L2CAP", "Could not close the connect socket", e)
        }
    }
}