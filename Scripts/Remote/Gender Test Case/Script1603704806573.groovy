import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

WS.sendRequest(findTestObject('Remote/Gender/Gender Create', [('uuid') : 'UUID11', ('code') : 'o', ('name') : 'Other', ('effectivedate') : '2020-01-01']), 
    FailureHandling.STOP_ON_FAILURE)

WS.sendRequest(findTestObject('Remote/Gender/Gender Read', [('uuid') : 'UUID11']), FailureHandling.STOP_ON_FAILURE)

WS.sendRequest(findTestObject('Remote/Gender/Gender Update', [('uuid') : 'UUID11', ('code') : 'o', ('name') : 'Gender Other Changed'
            , ('effectivedate') : '2020-01-06']), FailureHandling.STOP_ON_FAILURE)

WS.sendRequest(findTestObject('Remote/Gender/Gender List All', [('uuid') : 'UUID21']), FailureHandling.STOP_ON_FAILURE)

WS.sendRequest(findTestObject('Remote/Gender/Gender Delete', [('uuid') : 'UUID11']), FailureHandling.STOP_ON_FAILURE)

